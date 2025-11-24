use std::sync::Arc;

use crate::{
    constant::MIN_COMPRESS_SIZE,
    controllers::{
        account_c::create_account,
        calculator_c::get_calculator_window,
        display_setting_c::{
            get_display_setting_window, update_background_color, update_background_type,
            upload_background_picture,
        },
        explorer_c::get_explorer_window,
        file_c::{delete_file, rename_file, update_file_desktop_position},
        flappy_bird_c::get_flappy_bird_window,
        folder_c::{
            create_folder, delete_folder, get_folder_input, rename_folder,
            update_folder_desktop_position,
        },
        music_c::get_music_player_window,
        screen_c::{create_screen_grid, get_screen},
        snake_c::get_snake_window,
        taskbar_c::get_taskbar_menu_files,
        txt_c::{create_txt, get_txt_input, get_txt_window},
        web_builder_c::{
            add_section, delete_node, download_website, edit_node, get_edit_node, get_selected_section, get_selected_template, get_web_builder, get_web_builder_review, get_web_builder_window, insert_node
        },
    },
    middlewares::{csrf_mw::csrf_middleware, log_mw::{request_log, response_log}, session_mw::session_middleware},
    models::state::AppState,
};
use axum::{
    Router,
    http::{HeaderValue, StatusCode, header},
    middleware::{from_fn, from_fn_with_state},
    response::IntoResponse,
    routing::{get, post},
};

use deadpool_postgres::Pool;
use memory_serve::{MemoryServe, load_assets};
use papaya::HashMap;
use tower_http::{
    CompressionLevel,
    compression::{CompressionLayer, DefaultPredicate, Predicate, predicate::SizeAbove},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Route Not Found")
}


pub async fn create_router(pool: Pool) -> Router {
    let memory_router = MemoryServe::new(load_assets!("assets")).into_router();

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
            .route("/screen", post(create_screen_grid))
            .route("/account", post(create_account))
            .route(
                "/web_builder/{builder_id}/node/insert/{parent_node_id}",
                post(insert_node),
            )
            .route(
                "/web_builder/{builder_id}/section/add/{section_type}/{template_number}",
                post(add_section),
            )
            .route(
                "/web_builder/{builder_id}/download",
                post(download_website),
            )
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
            .route("/file/rename/{file_type}/{file_id}", post(rename_file))
            .route("/folder/rename/{folder_id}", post(rename_folder))
            .route(
                "/setting/display/background_type/{background_type}",
                post(update_background_type),
            )
            .route(
                "/setting/display/background_color/{background_color}",
                post(update_background_color),
            )
            .route(
                "/setting/display/background_picture",
                post(upload_background_picture),
            )
            .route(
                "/web_builder/{builder_id}/node/edit/{node_id}",
                post(edit_node),
            )
            .layer(from_fn(csrf_middleware)),
    );

    let delete_routes = Router::new().nest(
        "/delete",
        Router::new()
            .route("/file/{file_id}", post(delete_file))
            .route("/folder/{folder_id}", post(delete_folder))
            .route(
                "/web_builder/{builder_id}/node/delete/{node_id}",
                post(delete_node),
            )
            .layer(from_fn(csrf_middleware)),
    );

    let read_routes = Router::new().nest(
        "/read",
        Router::new()
            .route("/taskbar/files", get(get_taskbar_menu_files))
            .route(
                "/file/calculator/{height}/{width}",
                get(get_calculator_window),
            )
            .route("/file/snake/{height}/{width}", get(get_snake_window))
            .route(
                "/file/flappybird/{height}/{width}",
                get(get_flappy_bird_window),
            )
            .route("/file/music/{height}/{width}", get(get_music_player_window))
            .route("/txt/{file_id}/{height}/{width}", get(get_txt_window))
            .route("/txt/input/{file_id}", get(get_txt_input))
            .route("/folder/input/{folder_id}", get(get_folder_input))
            .route(
                "/folder/explorer/{folder_type}/{folder_id}/{height}/{width}/{open_new_task}/{previous_folder_id}",
                get(get_explorer_window),
            )
            .route(
                "/setting/display/{height}/{width}",
                get(get_display_setting_window),
            )
            .route(
                "/web_builder/{file_id}/{height}/{width}",
                get(get_web_builder_window),
            )
            .route(
                "/web_builder/{builder_id}",
                get(get_web_builder),
            )
            .route("/web_builder/{builder_id}/edit_node/{node_id}", get(get_edit_node))
            .route("/web_builder/section/{section_type}", get(get_selected_section))
            .route("/web_builder/template/{section_type}/{template_index}", get(get_selected_template))
            .route("/web_builder/view_website/{builder_id}", get(get_web_builder_review))
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
