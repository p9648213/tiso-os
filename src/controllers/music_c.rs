use axum::{Extension, extract::Path, response::IntoResponse};

use crate::{
    middlewares::session_mw::UserId, models::error::AppError, utilities::general::parse_user_id,
    views::music_v::render_music_player_window,
};

pub async fn get_music_player_window(
    Path((height, width)): Path<(i32, i32)>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let _ = parse_user_id(user_id)?;

    Ok((
        [(
            "HX-Trigger",
            r#"{"openFile":{"image":"/assets/images/music.svg", "window_id": "music-player-window"}}"#,
        )],
        render_music_player_window(height, width),
    ))
}
