use axum::{Extension, extract::Path, response::IntoResponse};

use crate::{
    middlewares::session_mw::UserId, models::error::AppError, utilities::common::parse_user_id,
    views::snake_v::render_snake_window,
};

pub async fn get_snake_window(
    Path((height, width)): Path<(i32, i32)>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let _ = parse_user_id(user_id)?;

    Ok((
        [(
            "HX-Trigger",
            r#"{"openFile":{"image":"/assets/images/snake/snake.svg", "window_id": "snake-canvas-container"}}"#,
        )],
        render_snake_window(height, width),
    ))
}
