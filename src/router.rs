use std::sync::Arc;

use crate::{
    contanst::MIN_COMPRESS_SIZE,
    controllers::{
        account_c::create_account,
        file_c::{delete_file, update_file_desktop_position},
        folder_c::{create_folder, delete_folder, update_folder_desktop_position},
        screen_c::{create_screen_grid, get_screen},
        txt_c::{create_txt, get_txt_input},
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

    let create_routes = Router::new().nest(
        "/create",
        Router::new()
            .route("/txt/{folder_id}/{position_id}", post(create_txt))
            .route("/folder/{folder_id}/{position_id}", post(create_folder))
            .route("/grid", post(create_screen_grid))
            .route("/account", post(create_account))
            .layer(from_fn(csrf_middleware)),
    );

    let update_routes = Router::new().nest(
        "/update",
        Router::new()
            .route(
                "/file/position/{file_id}/{desktop_id}/{position}",
                post(update_file_desktop_position),
            )
            .route(
                "/folder/position/{folder_id}/{desktop_id}/{position}",
                post(update_folder_desktop_position),
            )
            .layer(from_fn(csrf_middleware)),
    );

    let delete_routes = Router::new().nest(
        "/delete",
        Router::new()
            .route("/file/{file_id}", post(delete_file))
            .route("/folder/{folder_id}", post(delete_folder))
            .layer(from_fn(csrf_middleware)),
    );

    let read_routes = Router::new().nest(
        "/read",
        Router::new().route("/txt/input/{file_id}", get(get_txt_input)),
    );

    Router::new()
        .route("/", get(get_screen))
        .merge(create_routes)
        .merge(update_routes)
        .merge(delete_routes)
        .merge(read_routes)
        .layer(from_fn_with_state(app_state.clone(), session_middleware))
        .with_state(app_state.clone())
        .layer(compression_layer)
        .nest("/assets", memory_router)
        .layer(cache_control_layer)
        .fallback(fallback)
        .layer(TraceLayer::new_for_http().on_response(response_log))
        .layer(from_fn(request_log))
}
