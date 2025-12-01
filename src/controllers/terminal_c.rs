use axum::{
    Extension,
    extract::{Path, State, WebSocketUpgrade, ws::WebSocket},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, user_db::User},
    utilities::{common::parse_user_id, terminal_u::CommandLine},
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
            render_terminal_window(&username, height, width),
        ))
    } else {
        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting user",
        ))
    }
}

pub async fn terminal_ws_handler(
    ws: WebSocketUpgrade,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;
    Ok(ws.on_upgrade(move |socket| terminal_socket_handler(socket, user_id)))
}

pub async fn terminal_socket_handler(mut socket: WebSocket, user_id: i32) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            let command_line = CommandLine::from(msg.to_text().unwrap_or_default());
            let output = command_line.execute();
            serde_json::to_string(&output).unwrap()
        } else {
            return;
        };

        if socket.send(msg.into()).await.is_err() {
            return;
        }
    }
}
