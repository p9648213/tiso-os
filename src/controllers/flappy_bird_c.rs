use axum::{Extension, extract::Path, response::IntoResponse};
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId, models::error::AppError, utilities::user_utils::parse_user_id,
    views::flappy_bird_v::render_flappy_bird_window,
};

pub async fn get_flappy_bird_window(
    Path((height, width)): Path<(i32, i32)>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let _ = parse_user_id(user_id)?;

    Ok((
        [(
            "HX-Trigger",
            r#"{"openFile":{"image":"/assets/images/flappy_bird.png", "window_id": "flappy-canvas-container"}}"#,
        )],
        render_flappy_bird_window(height, width).render(),
    ))
}
