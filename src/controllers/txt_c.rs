use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        error::AppError,
        file_db::{File, FileType},
        txt_db::Txt,
        txt_window::TxtWindow,
    },
    utilities::user_utils::parse_user_id,
    views::txt_v::{render_txt_file, render_txt_input, render_txt_window},
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

    let row = File::create_file(
        user_id,
        folder_id,
        "New Text",
        FileType::Txt,
        desktop_position,
        &pool,
    )
    .await?;

    let file = File::try_from(&row, None);

    let file_id = file.id.ok_or_else(|| {
        tracing::error!("No id column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Txt::create_txt(file_id, &pool).await?;

    Ok(render_txt_file(file_id, &None, &None).render())
}

pub async fn get_txt_input(
    Path(file_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let row = File::get_file(file_id, user_id, vec!["file_name"], &pool).await?;

    let file = File::try_from(&row, None);

    let file_name = file.file_name.ok_or_else(|| {
        tracing::error!("No file_name column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(render_txt_input(file_id, &file_name).render())
}

pub async fn get_txt_window(
    Path((file_id, height, width)): Path<(i32, i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let row =
        TxtWindow::get_txt_window(file_id, user_id, vec!["id"], vec!["id", "file_name"], &pool)
            .await?;

    let txt_window = TxtWindow::try_from(&row);

    let file_name = txt_window.file.file_name.ok_or_else(|| {
        tracing::error!("No file_name column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let txt_id = txt_window.txt.id.ok_or_else(|| {
        tracing::error!("No id column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok((
        [(
            "HX-Trigger",
            format!(
                r#"{{"openFile":{{"image":"/assets/images/text-editor.svg", "window_id": "txt-window-{}"}}}}"#,
                txt_id
            ),
        )],
        render_txt_window(&file_name, txt_id, height, width).render(),
    ))
}
