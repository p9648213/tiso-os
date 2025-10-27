use axum::{Extension, extract::Path, response::IntoResponse};

use crate::{
    middlewares::session_mw::UserId, models::error::AppError, utilities::common::parse_user_id,
    views::calculator_v::render_calculator_window,
};

pub async fn get_calculator_window(
    Path((height, width)): Path<(i32, i32)>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let _ = parse_user_id(user_id)?;

    Ok((
        [(
            "HX-Trigger",
            r#"{"openFile":{"image":"/assets/images/calculator/calculator.svg", "window_id": "calculator-window"}}"#,
        )],
        render_calculator_window(height, width),
    ))
}
