use log::error;
use opentelemetry::global;
use opentelemetry_jaeger_propagator::Propagator;
use opentelemetry_otlp::{Protocol, SpanExporterBuilder, WithExportConfig};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{Config, TracerProvider};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_NAMESPACE};
use opentelemetry_stdout::SpanExporter;
use parking_lot::Mutex;
use std::cell::OnceCell;
use std::fs;
use std::sync::OnceLock;
use std::time::Duration;

pub enum ContextPropagationFormat {
    Jaeger,
    W3C,
}

#[derive(Clone)]
pub struct Identity {
    pub key: String,
    pub certificate: String,
}

#[derive(Clone)]
pub struct ClientTlsConfig {
    pub certificate: Option<String>,
    pub identity: Option<Identity>,
}

#[derive(Clone)]
pub struct TracerConfiguration {
    pub service_name: String,
    pub protocol: Protocol,
    pub endpoint: String,
    pub tls: Option<ClientTlsConfig>,
    pub timeout: Option<Duration>,
}

pub struct TelemetryConfiguration {
    pub context_propagation_format: Option<ContextPropagationFormat>,
    pub tracer: Option<TracerConfiguration>,
}

impl TelemetryConfiguration {
    pub fn no_op() -> Self {
        Self {
            context_propagation_format: Some(ContextPropagationFormat::W3C),
            tracer: None,
        }
    }
}

struct Configurator;

impl Configurator {
    fn new(config: &TelemetryConfiguration) -> Self {
        let tracer_provider = match config.tracer.as_ref() {
            Some(tracer_config) => {
                let exporter: SpanExporterBuilder = match tracer_config.protocol {
                    Protocol::Grpc => {
                        let mut builder = opentelemetry_otlp::new_exporter()
                            .tonic()
                            .with_endpoint(tracer_config.endpoint.clone());

                        builder = if let Some(timeout) = tracer_config.timeout {
                            builder.with_timeout(timeout)
                        } else {
                            builder
                        };

                        let mut tonic_tls_config =
                            tonic::transport::ClientTlsConfig::new().with_enabled_roots();

                        tonic_tls_config = if let Some(tls_config) = tracer_config.tls.as_ref() {
                            tonic_tls_config =
                                if let Some(root_certificate) = tls_config.certificate.as_ref() {
                                    let buf = fs::read(root_certificate)
                                        .expect("Failed to load root certificate");
                                    let cert = tonic::transport::Certificate::from_pem(buf);

                                    tonic_tls_config.ca_certificate(cert)
                                } else {
                                    tonic_tls_config
                                };

                            tonic_tls_config =
                                if let Some(identity_conf) = tls_config.identity.as_ref() {
                                    let cert = fs::read(&identity_conf.certificate)
                                        .expect("Failed to load identity certificate");
                                    let key = fs::read(&identity_conf.key)
                                        .expect("Failed to load identity key");
                                    let identity = tonic::transport::Identity::from_pem(cert, key);

                                    tonic_tls_config.identity(identity)
                                } else {
                                    tonic_tls_config
                                };

                            tonic_tls_config
                        } else {
                            tonic_tls_config
                        };
                        builder = builder.with_tls_config(tonic_tls_config);

                        builder.into()
                    }
                    Protocol::HttpBinary | Protocol::HttpJson => {
                        let mut builder = opentelemetry_otlp::new_exporter()
                            .http()
                            .with_endpoint(tracer_config.endpoint.clone())
                            .with_protocol(tracer_config.protocol);

                        builder = if let Some(timeout) = tracer_config.timeout {
                            builder.with_timeout(timeout)
                        } else {
                            builder
                        };

                        let mut client_builder = reqwest::Client::builder()
                            .tls_built_in_root_certs(true)
                            .use_rustls_tls();
                        client_builder = if let Some(tls_config) = tracer_config.tls.as_ref() {
                            client_builder =
                                if let Some(certificate) = tls_config.certificate.as_ref() {
                                    let buf = fs::read(certificate)
                                        .expect("Failed to read root certificate");
                                    let cert = reqwest::Certificate::from_pem(&buf)
                                        .expect("Failed to load root certificate");

                                    client_builder.add_root_certificate(cert)
                                } else {
                                    client_builder
                                };

                            client_builder = if let Some(identity) = tls_config.identity.as_ref() {
                                let mut buf = Vec::new();
                                buf.append(
                                    &mut fs::read(&identity.key)
                                        .expect("Failed to read identity key"),
                                );
                                buf.append(
                                    &mut fs::read(&identity.certificate)
                                        .expect("Failed to read identity certificate"),
                                );

                                let identity = reqwest::Identity::from_pem(&buf)
                                    .expect("Failed to load identity");

                                client_builder.identity(identity)
                            } else {
                                client_builder
                            };
                            client_builder
                        } else {
                            client_builder
                        };

                        let client = client_builder.build().expect("Failed to create a client");
                        builder = builder.with_http_client(client);
                        builder.into()
                    }
                };

                let runtime = RUNTIME.get_or_init(|| {
                    tokio::runtime::Runtime::new().expect("Failed to create runtime")
                });
                runtime.block_on(async {
                    opentelemetry_otlp::new_pipeline()
                        .tracing()
                        .with_exporter(exporter)
                        .with_trace_config(Config::default().with_resource(Resource::new(vec![
                            opentelemetry::KeyValue::new(
                                SERVICE_NAME,
                                tracer_config.service_name.clone(),
                            ),
                            opentelemetry::KeyValue::new(SERVICE_NAMESPACE, "savant-core"),
                        ])))
                        .install_batch(runtime::Tokio)
                        .expect("Failed to install OpenTelemetry tracer globally")
                })
            }
            None => TracerProvider::builder()
                .with_simple_exporter(SpanExporter::default())
                .build(),
        };
        global::set_tracer_provider(tracer_provider);

        match config.context_propagation_format {
            None | Some(ContextPropagationFormat::Jaeger) => {
                global::set_text_map_propagator(Propagator::new())
            }
            Some(ContextPropagationFormat::W3C) => {
                global::set_text_map_propagator(TraceContextPropagator::new())
            }
        }

        global::set_error_handler(|e| {
            error!(target: "opentelemetry", "{}", e);
        })
        .expect("Failed to set OpenTelemetry error handler");

        Self
    }

    fn shutdown(&mut self) {
        global::shutdown_tracer_provider();
    }
}

static CONFIGURATOR: Mutex<OnceCell<Configurator>> = Mutex::new(OnceCell::new());
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub fn init(config: &TelemetryConfiguration) {
    let configurator = CONFIGURATOR.lock();
    match configurator.get() {
        Some(_) => panic!("Open Telemetry has been configured"),
        None => {
            let result = configurator.set(Configurator::new(config));
            if result.is_err() {
                // should not happen
                panic!("Failed to configure OpenTelemetry");
            }
        }
    }
}

pub fn shutdown() {
    let mut configurator = CONFIGURATOR.lock();
    if let Some(mut c) = configurator.take() {
        c.shutdown()
    }
}
