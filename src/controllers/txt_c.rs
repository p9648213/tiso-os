use axum::{
    Extension,
    extract::{Path, State},
    response::IntoResponse,
};
use deadpool_postgres::Pool;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        error::AppError,
        file_db::{File, FileType},
        txt_db::Txt,
        txt_window::TxtWindow,
    },
    utilities::general::parse_user_id, views::txt_v_2::{render_txt_file, render_txt_input, render_txt_window},
};

pub async fn create_txt(
    Path((folder_id, position_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let desktop_position = if position_id == "-1" {
        None
    } else {
        Some(position_id)
    };

    let file = File::create_file(
        user_id,
        folder_id,
        "New Text",
        FileType::Txt,
        desktop_position,
        &pool,
    )
    .await?;

    let file_id = file.id.unwrap();

    Txt::create_txt(file_id, &pool).await?;

    Ok(render_txt_file(file_id, None, None))
}

pub async fn get_txt_input(
    Path(file_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let file = File::get_file(file_id, user_id, vec!["file_name"], &pool).await?;

    Ok(render_txt_input(file_id, &file.file_name.unwrap()))
}

pub async fn get_txt_window(
    Path((file_id, height, width)): Path<(i32, i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let txt_window =
        TxtWindow::get_txt_window(file_id, user_id, vec!["id"], vec!["id", "file_name"], &pool)
            .await?;
    let txt_id = txt_window.txt.id.unwrap();

    Ok((
        [(
            "HX-Trigger",
            format!(
                r#"{{"openFile":{{"image":"/assets/images/text-editor.svg", "window_id": "txt-window-{}"}}}}"#,
                txt_id
            ),
        )],
        render_txt_window(&txt_window.file.file_name.unwrap(), txt_id, height, width),
    ))
}
