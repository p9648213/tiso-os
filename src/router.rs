use crate::{
    controllers::screen_c::{create_folder, create_screen_grid, create_txt, get_screen},
    middlewares::{auth_mw::auth_middleware, csrf_mw::csrf_middleware, log_mw::request_log},
    models::state::AppState,
};
use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, header},
    middleware::from_fn,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use memory_serve::MemoryServe;
use tower_http::{
    compression::CompressionLayer, set_header::SetResponseHeaderLayer, trace::TraceLayer,
};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};
use tracing::Span;

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

fn response_log(response: &Response<Body>, latency: std::time::Duration, _: &Span) {
    tracing::info!("<- Response: status {} in {:?}", response.status(), latency)
}

pub async fn create_router() -> Router {
    let memory_router = MemoryServe::from_env().into_router();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let app_state = AppState {};

    let action_routes = Router::new().nest(
        "/action",
        Router::new()
            .route("/create-txt", post(create_txt))
            .route("/create-folder", post(create_folder))
            .route("/create-grid", post(create_screen_grid)),
    );

    Router::new()
        .route("/", get(get_screen))
        .merge(action_routes)
        .layer(from_fn(csrf_middleware))
        .layer(from_fn(auth_middleware))
        .layer(session_layer)
        .with_state(app_state.clone())
        .layer(CompressionLayer::new())
        .nest("/assets", memory_router)
        .layer(cache_control_layer)
        .fallback(fallback)
        .layer(TraceLayer::new_for_http().on_response(response_log))
        .layer(from_fn(request_log))
}
