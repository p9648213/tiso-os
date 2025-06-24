use std::sync::Arc;

use crate::{
    contanst::MIN_COMPRESS_SIZE,
    controllers::{
        account_c::create_account,
        file_c::update_file_desktop_position,
        folder_c::{create_folder, update_folder_desktop_position},
        screen_c::{create_screen_grid, get_screen},
        txt_c::create_txt,
    },
    middlewares::{csrf_mw::csrf_middleware, log_mw::request_log, session_mw::session_middleware},
    models::state::AppState,
};
use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, header},
    middleware::{from_fn, from_fn_with_state},
    response::{IntoResponse, Response},
    routing::{get, post},
};

use deadpool_postgres::Pool;
use memory_serve::MemoryServe;
use papaya::HashMap;
use tower_http::{
    CompressionLevel,
    compression::{CompressionLayer, DefaultPredicate, Predicate, predicate::SizeAbove},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing::Span;

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

fn response_log(response: &Response<Body>, latency: std::time::Duration, _: &Span) {
    tracing::info!("<- Response: status {} in {:?}", response.status(), latency)
}

pub async fn create_router(pool: Pool) -> Router {
    let memory_router = MemoryServe::from_env().into_router();

    let compression_layer = CompressionLayer::new()
        .quality(CompressionLevel::Fastest)
        .compress_when(DefaultPredicate::new().and(SizeAbove::new(MIN_COMPRESS_SIZE)));

    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    let app_state = AppState {
        session_map: Arc::new(HashMap::new()),
        pool,
    };

    let action_routes = Router::new().nest(
        "/action",
        Router::new()
            .route("/create-txt/{folder_id}/{position_id}", post(create_txt))
            .route(
                "/create-folder/{folder_id}/{position_id}",
                post(create_folder),
            )
            .route("/create-grid", post(create_screen_grid))
            .route("/create-account", post(create_account))
            .route(
                "/update-file-position/{file_id}/{desktop_id}/{position}",
                post(update_file_desktop_position),
            )
            .route(
                "/update-folder-position/{folder_id}/{desktop_id}/{position}",
                post(update_folder_desktop_position),
            )
            .layer(from_fn(csrf_middleware)),
    );

    Router::new()
        .route("/", get(get_screen))
        .merge(action_routes)
        .layer(from_fn_with_state(app_state.clone(), session_middleware))
        .with_state(app_state.clone())
        .layer(compression_layer)
        .nest("/assets", memory_router)
        .layer(cache_control_layer)
        .fallback(fallback)
        .layer(TraceLayer::new_for_http().on_response(response_log))
        .layer(from_fn(request_log))
}
