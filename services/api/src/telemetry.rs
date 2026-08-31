use opentelemetry::global;
use opentelemetry_sdk::trace::{Config, Sampler};
use opentelemetry_sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use tracing::info;

pub fn init_tracer(service_name: &str, otlp_endpoint: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Initializing OpenTelemetry tracer for service '{}' -> {}", service_name, otlp_endpoint);

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otlp_endpoint);

    let _tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", service_name.to_string()),
                    KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                    KeyValue::new("deployment.environment", "production"),
                ])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    info!("OpenTelemetry tracer pipeline initialized successfully.");
    Ok(())
}

pub fn shutdown_tracer() {
    global::shutdown_tracer_provider();
}
