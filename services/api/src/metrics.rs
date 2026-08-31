use prometheus::{
    register_counter_vec_with_registry, register_gauge_with_registry, CounterVec, Encoder, Gauge,
    Registry, TextEncoder,
};
use std::sync::OnceLock;

pub struct AppMetrics {
    pub registry: Registry,
    pub http_requests_total: CounterVec,
    pub active_lab_sessions: Gauge,
    pub terminal_ws_connections: Gauge,
}

static METRICS: OnceLock<AppMetrics> = OnceLock::new();

pub fn get_metrics() -> &'static AppMetrics {
    METRICS.get_or_init(|| {
        let registry = Registry::new();

        let http_requests_total = register_counter_vec_with_registry!(
            "kubelab_http_requests_total",
            "Total number of HTTP requests processed by KubeLab API Gateway.",
            &["service", "status"],
            registry
        )
        .expect("Failed to register http_requests_total metric");

        let active_lab_sessions = register_gauge_with_registry!(
            "kubelab_active_sessions",
            "Current number of active lab sandbox namespaces.",
            registry
        )
        .expect("Failed to register active_lab_sessions metric");

        let terminal_ws_connections = register_gauge_with_registry!(
            "kubelab_terminal_ws_connections",
            "Current number of open interactive WebSocket terminal sessions.",
            registry
        )
        .expect("Failed to register terminal_ws_connections metric");

        // Initial default metric observations
        http_requests_total.with_label_values(&["kubelab-api", "200"]).inc_by(1.0);
        active_lab_sessions.set(1.0);
        terminal_ws_connections.set(0.0);

        AppMetrics {
            registry,
            http_requests_total,
            active_lab_sessions,
            terminal_ws_connections,
        }
    })
}

pub fn render_prometheus_metrics() -> String {
    let metrics = get_metrics();
    let encoder = TextEncoder::new();
    let metric_families = metrics.registry.gather();
    let mut buffer = Vec::new();
    encoder
        .encode(&metric_families, &mut buffer)
        .unwrap_or_default();
    String::from_utf8(buffer).unwrap_or_default()
}
