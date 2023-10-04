use anyhow::Result;
use rocksdb::{Options, DB};
pub struct PersistentQueue {
    db: DB,
    path: String,
    write_index: u128,
    read_index: u128,
    max_elements: u128,
}

const U128_BYTE_LEN: usize = 16;
const WRITE_INDEX_CELL: u128 = u128::MAX;
const READ_INDEX_CELL: u128 = u128::MAX - 1;

#[cfg(test)]
const MAX_ALLOWED_INDEX: u128 = 4;
#[cfg(not(test))]
const MAX_ALLOWED_INDEX: u128 = u128::MAX - 2;

impl PersistentQueue {
    pub fn new(path: String, max_elements: u128) -> Result<Self> {
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.set_prefix_extractor(rocksdb::SliceTransform::create_fixed_prefix(U128_BYTE_LEN));

        let db = DB::open(&db_opts, &path)?;

        let write_index_opt = db.get(Self::index_to_key(WRITE_INDEX_CELL))?;
        let write_index = match write_index_opt {
            Some(v) => {
                let mut buf = [0u8; U128_BYTE_LEN];
                buf.copy_from_slice(&v);
                u128::from_le_bytes(buf)
            }
            None => 0u128,
        };

        let read_index_opt = db.get(Self::index_to_key(READ_INDEX_CELL))?;
        let read_index = match read_index_opt {
            Some(v) => {
                let mut buf = [0u8; U128_BYTE_LEN];
                buf.copy_from_slice(&v);
                u128::from_le_bytes(buf)
            }
            None => 0u128,
        };

        Ok(Self {
            db,
            path,
            write_index,
            read_index,
            max_elements,
        })
    }

    fn index_to_key(index: u128) -> [u8; U128_BYTE_LEN] {
        index.to_le_bytes()
    }

    pub fn remove_db(path: String) -> Result<()> {
        Ok(DB::destroy(&Options::default(), path)?)
    }

    pub fn size(&self) -> Result<u64> {
        Ok(savant_utils::fs::dir_size(&self.path)?)
    }

    pub fn len(&self) -> u128 {
        if self.write_index >= self.read_index {
            self.write_index - self.read_index
        } else {
            MAX_ALLOWED_INDEX - self.read_index + self.write_index
        }
    }

    pub fn is_empty(&self) -> bool {
        self.write_index == self.read_index
    }

    pub fn push(&mut self, value: &[u8]) -> Result<()> {
        if self.write_index >= self.read_index + self.max_elements {
            return Err(anyhow::anyhow!("Queue is full"));
        }

        let mut batch = rocksdb::WriteBatch::default();

        batch.put(Self::index_to_key(self.write_index), value);

        self.write_index += 1;
        if self.write_index == MAX_ALLOWED_INDEX {
            self.write_index = 0;
        }

        batch.put(
            Self::index_to_key(WRITE_INDEX_CELL),
            &self.write_index.to_le_bytes(),
        );

        self.db.write(batch)?;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Option<Vec<u8>>> {
        if self.read_index == self.write_index {
            return Ok(None);
        }

        let key = Self::index_to_key(self.read_index);
        let value = self.db.get(key)?;
        let mut batch = rocksdb::WriteBatch::default();

        if let Some(v) = value {
            batch.delete(key);
            self.read_index += 1;
            if self.read_index == MAX_ALLOWED_INDEX {
                self.read_index = 0;
            }
            batch.put(
                Self::index_to_key(READ_INDEX_CELL),
                &self.read_index.to_le_bytes(),
            );
            self.db.write(batch)?;
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal_ops() {
        let path = "/tmp/test1".to_string();
        _ = PersistentQueue::remove_db(path.clone());
        {
            let mut db = PersistentQueue::new(path.clone(), 10).unwrap();
            db.push(&[1, 2, 3]).unwrap();
            db.push(&[4, 5, 6]).unwrap();
            assert_eq!(db.len(), 2);
            assert!(matches!(db.pop(), Ok(Some(_))));
            assert!(matches!(db.pop(), Ok(Some(_))));
            assert!(db.is_empty());
            db.push(&[1, 2, 3]).unwrap();
            db.push(&[4, 5, 6]).unwrap();
            db.push(&[7, 8, 9]).unwrap();
            assert_eq!(db.len(), 3);
            assert_eq!(db.read_index, 2);
            assert_eq!(db.write_index, 1);
            assert!(matches!(db.pop(), Ok(Some(_))));
            assert_eq!(db.len(), 2);
            assert_eq!(db.read_index, 3);
            assert_eq!(db.write_index, 1);
            assert!(matches!(db.pop(), Ok(Some(_))));
            assert_eq!(db.len(), 1);
            assert_eq!(db.read_index, 0);
            assert_eq!(db.write_index, 1);
            let data = db.pop().unwrap().unwrap();
            assert!(db.is_empty());
            assert_eq!(data, vec![7, 8, 9]);
        }
        PersistentQueue::remove_db(path).unwrap();
    }

    #[test]
    fn test_limit_len_ops() {
        let path = "/tmp/test2".to_string();
        _ = PersistentQueue::remove_db(path.clone());
        {
            let mut db = PersistentQueue::new(path.clone(), 2).unwrap();
            db.push(&[1, 2, 3]).unwrap();
            db.push(&[4, 5, 6]).unwrap();
            assert!(matches!(db.push(&[1, 2, 3]), Err(_)));
        }
        PersistentQueue::remove_db(path).unwrap();
    }
}