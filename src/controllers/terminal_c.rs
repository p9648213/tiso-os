use axum::{
    Extension,
    extract::{Path, State, WebSocketUpgrade, ws::WebSocket},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, state::SessionMap, user_db::User},
    utilities::{common::parse_user_id, terminal_u::CommandLine},
    views::terminal_v::render_terminal_window,
};

pub async fn get_terminal_window(
    Path((height, width)): Path<(i32, i32)>,
    State(pool): State<Pool>,
    State(session_map): State<SessionMap>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let row = User::get_user_by_id(user_id, vec!["username"], &pool).await?;

    if let Some(row) = row {
        let user = User::try_from(&row, None);
        let username = user.username.unwrap();

        let session_map = session_map.pin_owned();
        session_map.insert(format!("current-dir-{}", user_id), "/".to_string());

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
    State(session_map): State<SessionMap>,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;
    Ok(ws.on_upgrade(move |socket| terminal_socket_handler(socket, user_id, session_map, pool)))
}

pub async fn terminal_socket_handler(
    mut socket: WebSocket,
    user_id: i32,
    session_map: SessionMap,
    pool: Pool,
) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            let command_line = CommandLine::setup_command(
                msg.to_text().unwrap_or_default(),
                user_id,
                &session_map,
                &pool,
            );
            let output = command_line.execute().await;
            serde_json::to_string(&output).unwrap()
        } else {
            return;
        };

        if socket.send(msg.into()).await.is_err() {
            return;
        }
    }
}
