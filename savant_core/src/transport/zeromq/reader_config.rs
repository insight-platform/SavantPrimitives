use super::{
    parse_zmq_socket_uri, ReaderSocketType, SocketType, TopicPrefix, RECEIVE_HWM, RECEIVE_TIMEOUT,
    ROUTING_ID_CACHE_SIZE,
};
use anyhow::bail;
use savant_utils::default_once::DefaultOnceCell;

#[derive(Clone, Debug, Default)]
pub struct ReaderConfig(ReaderConfigBuilder);

impl ReaderConfig {
    pub fn new() -> anyhow::Result<ReaderConfigBuilder> {
        Ok(ReaderConfigBuilder::default())
    }

    pub fn endpoint(&self) -> &String {
        &self.0.endpoint.get_or_init()
    }

    pub fn socket_type(&self) -> &ReaderSocketType {
        &self.0.socket_type.get_or_init()
    }

    pub fn bind(&self) -> &bool {
        &self.0.bind.get_or_init()
    }

    pub fn receive_timeout(&self) -> &usize {
        &self.0.receive_timeout.get_or_init()
    }

    pub fn receive_hwm(&self) -> &usize {
        &self.0.receive_hwm.get_or_init()
    }

    pub fn topic_prefix(&self) -> &TopicPrefix {
        &self.0.topic_prefix.get_or_init()
    }

    pub fn routing_ids_cache_size(&self) -> &usize {
        &self.0.routing_ids_cache_size.get_or_init()
    }

    pub fn fix_ipc_permissions(&self) -> &bool {
        &self.0.fix_ipc_permissions.get_or_init()
    }
}

#[derive(Clone, Debug)]
pub struct ReaderConfigBuilder {
    endpoint: DefaultOnceCell<String>,
    socket_type: DefaultOnceCell<ReaderSocketType>,
    bind: DefaultOnceCell<bool>,
    receive_timeout: DefaultOnceCell<usize>,
    receive_hwm: DefaultOnceCell<usize>,
    topic_prefix: DefaultOnceCell<TopicPrefix>,
    routing_ids_cache_size: DefaultOnceCell<usize>,
    fix_ipc_permissions: DefaultOnceCell<bool>,
}

impl Default for ReaderConfigBuilder {
    fn default() -> Self {
        Self {
            endpoint: DefaultOnceCell::new(String::new()),
            socket_type: DefaultOnceCell::new(ReaderSocketType::Router),
            bind: DefaultOnceCell::new(true),
            receive_timeout: DefaultOnceCell::new(RECEIVE_TIMEOUT),
            receive_hwm: DefaultOnceCell::new(RECEIVE_HWM),
            topic_prefix: DefaultOnceCell::new(TopicPrefix::None),
            routing_ids_cache_size: DefaultOnceCell::new(ROUTING_ID_CACHE_SIZE),
            fix_ipc_permissions: DefaultOnceCell::new(true),
        }
    }
}

impl ReaderConfigBuilder {
    pub fn build(self) -> anyhow::Result<ReaderConfig> {
        if self.endpoint.get_or_init().is_empty() {
            bail!("ZeroMQ endpoint is not set");
        }
        Ok(ReaderConfig(self))
    }
    pub fn url(self, url: &str) -> anyhow::Result<Self> {
        let uri = parse_zmq_socket_uri(url.to_string())?;
        self.endpoint.set(uri.endpoint)?;
        if let Some(bind) = uri.bind {
            self.bind.set(bind)?;
        }
        if let Some(socket_type) = uri.socket_type {
            self.socket_type.set(match socket_type {
                SocketType::Reader(socket_type) => socket_type,
                _ => bail!("Invalid socket type for reader: {:?}", socket_type),
            })?;
        }
        Ok(self)
    }

    pub fn with_endpoint(self, endpoint: &str) -> anyhow::Result<Self> {
        self.endpoint.set(endpoint.to_string())?;
        Ok(self)
    }

    pub fn with_socket_type(self, socket_type: ReaderSocketType) -> anyhow::Result<Self> {
        self.socket_type.set(socket_type)?;
        Ok(self)
    }

    pub fn with_bind(self, bind: bool) -> anyhow::Result<Self> {
        self.bind.set(bind)?;
        Ok(self)
    }

    pub fn with_receive_timeout(self, receive_timeout: usize) -> anyhow::Result<Self> {
        self.receive_timeout.set(receive_timeout)?;
        Ok(self)
    }

    pub fn with_receive_hwm(self, receive_hwm: usize) -> anyhow::Result<Self> {
        self.receive_hwm.set(receive_hwm)?;
        Ok(self)
    }

    pub fn with_topic_prefix(self, topic_prefix: TopicPrefix) -> anyhow::Result<Self> {
        self.topic_prefix.set(topic_prefix)?;
        Ok(self)
    }

    pub fn with_routing_ids_cache_size(
        self,
        routing_ids_cache_size: usize,
    ) -> anyhow::Result<Self> {
        self.routing_ids_cache_size.set(routing_ids_cache_size)?;
        Ok(self)
    }

    pub fn with_fix_ipc_permissions(self, fix_ipc_permissions: bool) -> anyhow::Result<Self> {
        self.fix_ipc_permissions.set(fix_ipc_permissions)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::transport::zeromq::reader_config::ReaderConfig;
    use crate::transport::zeromq::ReaderSocketType;

    #[test]
    fn test_reader_config_with_endpoint() -> anyhow::Result<()> {
        let url = String::from("tcp:///abc");
        let config = ReaderConfig::new()?.with_endpoint(&url)?.build()?;
        assert_eq!(config.endpoint(), &url);
        Ok(())
    }

    #[test]
    fn test_duplicate_configuration_fails() -> anyhow::Result<()> {
        let config = ReaderConfig::new()?.with_endpoint("tcp:///abc")?;
        assert!(config.with_endpoint("tcp:///abc").is_err());
        Ok(())
    }

    #[test]
    fn test_build_empty_config_fails() -> anyhow::Result<()> {
        let config = ReaderConfig::new()?;
        assert!(config.build().is_err());
        Ok(())
    }

    #[test]
    fn test_full_uri() -> anyhow::Result<()> {
        let endpoint = String::from("ipc:///abc/def");
        let url = format!("sub+connect:{}", endpoint);
        let config = ReaderConfig::new()?.url(&url)?.build()?;
        assert_eq!(config.endpoint(), &endpoint);
        assert_eq!(config.bind(), &false);
        assert_eq!(config.socket_type(), &ReaderSocketType::Sub);
        Ok(())
    }

    #[test]
    fn test_writer_results_in_error() -> anyhow::Result<()> {
        let endpoint = String::from("ipc:///abc/def");
        let url = format!("pub+connect:{}", endpoint);
        let config = ReaderConfig::new()?.url(&url);
        assert!(config.is_err());
        Ok(())
    }
}