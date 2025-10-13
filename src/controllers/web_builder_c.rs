use axum::{
    Extension,
    extract::{Path, State},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, web_builder_window::WebBuilderWindow},
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
        vec!["id"],
        vec!["id", "file_name"],
        &pool,
    )
    .await?;

    let web_builder_id = web_builder_window.web_builder.id.unwrap();

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
            height,
            width,
        )
        .render(),
    ))
}
