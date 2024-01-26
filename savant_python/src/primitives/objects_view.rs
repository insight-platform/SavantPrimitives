use crate::match_query::MatchQuery;
use crate::primitives::object::BorrowedVideoObject;
use crate::release_gil;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use savant_core::match_query::*;
use std::collections::HashMap;
use std::sync::Arc;

pub type VideoObjectsViewBatch = HashMap<i64, VideoObjectsView>;

/// Determines which object bbox is a subject of the operation
///
#[pyclass]
#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub enum VideoObjectBBoxType {
    Detection,
    TrackingInfo,
}

impl From<VideoObjectBBoxType> for savant_core::primitives::object::VideoObjectBBoxType {
    fn from(value: VideoObjectBBoxType) -> Self {
        match value {
            VideoObjectBBoxType::Detection => {
                savant_core::primitives::object::VideoObjectBBoxType::Detection
            }
            VideoObjectBBoxType::TrackingInfo => {
                savant_core::primitives::object::VideoObjectBBoxType::TrackingInfo
            }
        }
    }
}

impl From<savant_core::primitives::object::VideoObjectBBoxType> for VideoObjectBBoxType {
    fn from(value: savant_core::primitives::object::VideoObjectBBoxType) -> Self {
        match value {
            savant_core::primitives::object::VideoObjectBBoxType::Detection => {
                VideoObjectBBoxType::Detection
            }
            savant_core::primitives::object::VideoObjectBBoxType::TrackingInfo => {
                VideoObjectBBoxType::TrackingInfo
            }
        }
    }
}

#[pyclass]
#[derive(Clone, Debug)]
#[repr(C)]
pub struct VideoObjectsView {
    pub(crate) inner: Arc<Vec<BorrowedVideoObject>>,
}

impl VideoObjectsView {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl From<Vec<BorrowedVideoObject>> for VideoObjectsView {
    fn from(value: Vec<BorrowedVideoObject>) -> Self {
        VideoObjectsView {
            inner: Arc::new(value),
        }
    }
}

impl From<Vec<savant_core::primitives::rust::BorrowedVideoObject>> for VideoObjectsView {
    fn from(value: Vec<savant_core::primitives::rust::BorrowedVideoObject>) -> Self {
        VideoObjectsView {
            inner: Arc::new(value.into_iter().map(BorrowedVideoObject).collect()),
        }
    }
}

#[pymethods]
impl VideoObjectsView {
    #[classattr]
    const __hash__: Option<Py<PyAny>> = None;

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __getitem__(&self, index: usize) -> PyResult<BorrowedVideoObject> {
        self.inner
            .get(index)
            .ok_or(PyIndexError::new_err("index out of range"))
            .map(|x| x.clone())
    }

    #[getter]
    pub fn memory_handle(&self) -> usize {
        self as *const Self as usize
    }

    #[getter]
    pub fn object_memory_handles(&self) -> Vec<usize> {
        self.inner.iter().map(|x| x.memory_handle()).collect()
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.inner.len())
    }

    #[getter]
    fn ids(&self) -> Vec<i64> {
        self.inner.iter().map(|x| x.get_id()).collect()
    }

    #[getter]
    pub fn track_ids(&self) -> Vec<Option<i64>> {
        self.inner
            .iter()
            .map(|o| o.get_track_id())
            .collect::<Vec<_>>()
    }

    #[getter]
    pub fn sorted_by_id(&self) -> VideoObjectsView {
        let mut objects = self.inner.as_ref().clone();
        objects.sort_by_key(|o| o.get_id());
        VideoObjectsView {
            inner: Arc::new(objects),
        }
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub(crate) struct QueryFunctions;

#[pymethods]
impl QueryFunctions {
    #[staticmethod]
    #[pyo3(name = "filter")]
    #[pyo3(signature = (v, q, no_gil = true))]
    pub(crate) fn filter_gil(
        v: &VideoObjectsView,
        q: &MatchQuery,
        no_gil: bool,
    ) -> VideoObjectsView {
        release_gil!(no_gil, || {
            let objs = v.inner.iter().map(|o| o.0.clone()).collect::<Vec<_>>();
            VideoObjectsView::from(filter(&objs, &q.0))
        })
    }

    // #[staticmethod]
    // #[pyo3(name = "batch_filter")]
    // #[pyo3(signature = (v, q, no_gil = true))]
    // pub(crate) fn batch_filter_gil(
    //     v: VideoObjectsViewBatch,
    //     q: &MatchQueryProxy,
    //     no_gil: bool,
    // ) -> VideoObjectsViewBatch {
    //     release_gil!(no_gil, || {
    //         let m = v
    //             .iter()
    //             .map(|(id, v)| (*id, v.inner.to_vec()))
    //             .collect::<HashMap<_, _>>();
    //         batch_filter(&m, &q.inner)
    //             .into_iter()
    //             .map(|(id, v)| (id, VideoObjectsView { inner: Arc::new(v) }))
    //             .collect::<VideoObjectsViewBatch>()
    //     })
    // }

    #[staticmethod]
    #[pyo3(name = "partition")]
    #[pyo3(signature = (v, q, no_gil = true))]
    pub(crate) fn partition_gil(
        v: &VideoObjectsView,
        q: &MatchQuery,
        no_gil: bool,
    ) -> (VideoObjectsView, VideoObjectsView) {
        release_gil!(no_gil, || {
            let objs = v.inner.iter().map(|o| o.0.clone()).collect::<Vec<_>>();
            let (a, b) = partition(&objs, &q.0);
            (a.into(), b.into())
        })
    }
}
