use axum::{
    Extension, extract::{Path, State}, http::StatusCode, response::IntoResponse
};
use deadpool_postgres::Pool;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, user_db::User},
    utilities::common::parse_user_id,
    views::terminal_v::render_terminal_window,
};

pub async fn get_terminal_window(
    Path((height, width)): Path<(i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let row = User::get_user_by_id(user_id, vec!["username"], &pool).await?;

    if let Some(row) = row {
        let user = User::try_from(&row, None);

        let username = user.username.unwrap();

        Ok((
            [(
                "HX-Trigger",
                r#"{"openFile":{"image":"/assets/images/terminal/terminal.svg", "window_id": "terminal-window"}}"#,
            )],
            render_terminal_window(&username, height, width)
        ))
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting user",
        ));
    }
}
