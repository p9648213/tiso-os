use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, web_builder_db::DomTree, web_builder_window::WebBuilderWindow},
    utilities::general::parse_user_id,
    views::web_builder_v::render_web_builder_window,
};

pub async fn get_web_builder_window(
    Path((file_id, height, width)): Path<(i32, i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let web_builder_window = WebBuilderWindow::get_web_builder_window(
        file_id,
        user_id,
        vec!["id", "data", "name"],
        vec!["id", "file_name"],
        &pool,
    )
    .await?;

    let web_builder_id = web_builder_window.web_builder.id.unwrap();
    let builder_name = web_builder_window.web_builder.name.unwrap();
    let data = web_builder_window.web_builder.data.unwrap();

    let dom_tree: DomTree = serde_json::from_value(data).map_err(|err| {
        tracing::error!("Could not parse dom tree: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    Ok((
        [(
            "HX-Trigger",
            format!(
                r#"{{"openFile":{{"image":"/assets/images/web-builder.svg", "window_id": "web-builder-window-{}"}}}}"#,
                web_builder_id
            ),
        )],
        render_web_builder_window(
            web_builder_id,
            &web_builder_window.file.file_name.unwrap(),
            &builder_name,
            &dom_tree,
            height,
            width,
        )
        .render(),
    ))
}
