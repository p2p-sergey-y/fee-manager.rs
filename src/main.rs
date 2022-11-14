mod mev;
use mev::{BLSPubkey, Config, Eth1Address};

use axum::{
    error_handling::HandleErrorLayer,
    extract::{ContentLengthLimit, Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{
    borrow::Cow,
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type SharedState = Arc<RwLock<Config>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "FEE_MANAGER=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Build our application by composing routes
    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/api/pubkey/:pubkey", get(pubkey_get).post(pubkey_set))
        .route("/api/mev", get(list_mev))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(SharedState::default()))
                .into_inner(),
        );

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn healthcheck() -> &'static str {
    "Application is live"
}

#[axum_macros::debug_handler]
async fn pubkey_get(
    Path(pubkey): Path<BLSPubkey>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    let db = &state.read().unwrap().db;

    if let Some(value) = db.get(&pubkey) {
        Ok(value.to_string())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[axum_macros::debug_handler]
async fn pubkey_set(
    Path(pubkey): Path<BLSPubkey>,
    ContentLengthLimit(str_address): ContentLengthLimit<String, { 1024 * 5_000 }>,
    Extension(state): Extension<SharedState>,
) -> Result<String, StatusCode> {
    if let Ok(eth1address) = Eth1Address::try_from(str_address) {
        state.write().unwrap().db.insert(pubkey, eth1address);
        Ok("Inserted".to_string())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn list_mev(
    Extension(state): Extension<SharedState>,
) -> Json<HashMap<BLSPubkey, Eth1Address>> {
    let db = &state.read().unwrap().db;
    Json(db.clone())
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}
