use kubelab_api::routes::create_routes;
use kubelab_api::state::AppState;
use kubelab_api::db::Database;
use kubelab_api::cache::Cache;
use kubelab_api::events::EventBus;
use kubelab_api::telemetry::{init_tracer, shutdown_tracer};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,kubelab=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 1. Initialize OpenTelemetry tracing if configured
    if let Ok(otlp_endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        if let Err(e) = init_tracer("kubelab-api", &otlp_endpoint) {
            tracing::warn!("Failed to initialize OpenTelemetry tracer: {:?}", e);
        }
    }

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "super_secure_kubelab_jwt_secret_key_32_chars!".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    // 2. Connect backing infrastructure services (PostgreSQL, Redis, NATS)
    let db = match std::env::var("DATABASE_URL") {
        Ok(url) => match Database::connect(&url).await {
            Ok(db) => Some(db),
            Err(e) => {
                tracing::warn!("PostgreSQL connection failed: {:?}. Using memory store.", e);
                None
            }
        },
        Err(_) => None,
    };

    let cache = match std::env::var("REDIS_URL") {
        Ok(url) => match Cache::connect(&url).await {
            Ok(cache) => Some(cache),
            Err(e) => {
                tracing::warn!("Redis connection failed: {:?}. Using memory cache.", e);
                None
            }
        },
        Err(_) => None,
    };

    let events = match std::env::var("NATS_URL") {
        Ok(url) => match EventBus::connect(&url).await {
            Ok(bus) => Some(bus),
            Err(e) => {
                tracing::warn!("NATS connection failed: {:?}. Using in-memory events.", e);
                None
            }
        },
        Err(_) => None,
    };

    let k8s_client = match kube::Client::try_default().await {
        Ok(client) => {
            tracing::info!("Connected to live Kubernetes cluster successfully");
            Some(client)
        }
        Err(e) => {
            tracing::info!("No live Kubernetes cluster detected ({:?}). Running with in-memory sandbox provisioner.", e);
            None
        }
    };

    let mut state = AppState::new(jwt_secret).with_backing_services(db, cache, events);
    if let Some(client) = k8s_client {
        state = state.with_k8s_client(client);
    }

    // Configurable CORS: defaults to localhost dev origins; set CORS_ORIGINS in production
    let cors = match std::env::var("CORS_ORIGINS") {
        Ok(origins) => {
            let allowed: Vec<_> = origins
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            tracing::info!("CORS: Restricted to {} origin(s)", allowed.len());
            CorsLayer::new()
                .allow_origin(allowed)
                .allow_methods(Any)
                .allow_headers(Any)
        }
        Err(_) => {
            // Development defaults — not wide-open Any
            let dev_origins = vec![
                "http://localhost:3000".parse().unwrap(),
                "http://localhost:8080".parse().unwrap(),
                "http://127.0.0.1:3000".parse().unwrap(),
            ];
            tracing::info!("CORS: Using development defaults (localhost only)");
            CorsLayer::new()
                .allow_origin(dev_origins)
                .allow_methods(Any)
                .allow_headers(Any)
        }
    };

    let app = create_routes(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    tracing::info!("🚀 KubeLab API Gateway listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    shutdown_tracer();
    Ok(())
}
