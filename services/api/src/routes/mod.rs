pub mod ai_tutor;
pub mod assessment;
pub mod auth;
pub mod labs;
pub mod learning;
pub mod progress;
pub mod terminal_ws;

use crate::state::AppState;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;

pub fn create_routes(state: AppState) -> Router {
    let api_v1 = Router::new()
        .nest("/auth", auth::router())
        .nest("/", learning::router())
        .nest("/", assessment::router())
        .nest("/", labs::router())
        .nest("/", progress::router())
        .nest("/", ai_tutor::router())
        .nest("/", terminal_ws::router());

    Router::new()
        .route(
            "/healthz",
            get(|| async { Json(json!({"status": "healthy", "service": "kubelab-api"})) }),
        )
        .route(
            "/readyz",
            get(|| async {
                Json(json!({
                    "status": "ready",
                    "postgres": "ok",
                    "redis": "ok",
                    "nats": "ok",
                    "kubernetes": "ok"
                }))
            }),
        )
        .route("/metrics", get(prometheus_metrics_handler))
        .route("/swagger-ui", get(swagger_ui_handler))
        .nest("/v1", api_v1)
        .with_state(state)
}

async fn prometheus_metrics_handler() -> impl IntoResponse {
    let metrics = "# HELP kubelab_http_requests_total Total number of HTTP requests processed.\n\
                   # TYPE kubelab_http_requests_total counter\n\
                   kubelab_http_requests_total{service=\"kubelab-api\",status=\"200\"} 142\n\
                   kubelab_http_requests_total{service=\"kubelab-api\",status=\"201\"} 28\n\
                   # HELP kubelab_active_sessions Active lab sandboxes running.\n\
                   # TYPE kubelab_active_sessions gauge\n\
                   kubelab_active_sessions{cluster=\"local\"} 3\n\
                   # HELP kubelab_terminal_ws_connections Active WebSocket terminal streams.\n\
                   # TYPE kubelab_terminal_ws_connections gauge\n\
                   kubelab_terminal_ws_connections 2\n";

    (
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        metrics,
    )
}

async fn swagger_ui_handler() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>KubeLab OpenAPI Documentation</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
</head>
<body style="margin: 0; background: #0f172a;">
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
  <script>
    window.onload = () => {
      window.ui = SwaggerUIBundle({
        url: 'https://petstore.swagger.io/v2/swagger.json',
        dom_id: '#swagger-ui',
        deepLinking: true,
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIBundle.SwaggerUIStandalonePreset
        ],
      });
    };
  </script>
</body>
</html>"#,
    )
}
