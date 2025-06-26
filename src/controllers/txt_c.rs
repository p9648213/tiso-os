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
    models::{error::AppError, files_db::File},
    views::txt_v::{render_txt, render_txt_input},
};

pub async fn create_txt(
    Path((folder_id, position_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let desktop_position = if position_id == "-1" {
        None
    } else {
        Some(position_id)
    };

    let row = File::create_file(
        user_id,
        folder_id,
        "New Text",
        &format!("/execute/txt/{}", user_id),
        desktop_position,
        &pool,
    )
    .await?;

    let file = File::try_from(&row, None);

    let file_id = file.id.ok_or_else(|| {
        tracing::error!("No id column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(render_txt(file_id).render())
}

pub async fn get_txt_input(Path((file_id, file_name)): Path<(i32, String)>) -> impl IntoResponse {
    render_txt_input(file_id, &file_name).render()
}
