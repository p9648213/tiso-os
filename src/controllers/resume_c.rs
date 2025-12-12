use axum::{Extension, extract::Path, response::IntoResponse};

use crate::{
    middlewares::session_mw::UserId, models::error::AppError, utilities::common::parse_user_id,
    views::resume_v::render_resume_window,
};

pub async fn get_resume_window(
    Path((height, width)): Path<(i32, i32)>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let _ = parse_user_id(user_id)?;

    Ok((
        [(
            "HX-Trigger",
            r#"{"openFile":{"image":"/assets/images/resume/resume.svg", "window_id": "resume-window"}}"#,
        )],
        render_resume_window(height, width),
    ))
}
