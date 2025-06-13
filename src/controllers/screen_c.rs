use axum::{Extension, Form, extract::State, http::StatusCode, response::IntoResponse};
use deadpool_postgres::Pool;
use hypertext::Renderable;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, files_db::File, folders_db::Folder},
    views::screen_v::{render_screen, render_screen_grid, render_welcome_screen},
};

#[derive(Deserialize)]
pub struct GridForm {
    pub height: u16,
    pub width: u16,
}

pub async fn get_screen(Extension(user_id): Extension<UserId>) -> impl IntoResponse {
    if user_id.0.is_none() {
        return render_welcome_screen().render();
    }
    render_screen().render()
}

pub async fn create_screen_grid(
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    Form(form): Form<GridForm>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let rows = Folder::get_desktop_folders(user_id, vec!["id"], &pool).await?;

    let desktop_folder = Folder::try_from(&rows, None);

    let desktop_id = desktop_folder.id.ok_or_else(|| {
        tracing::error!("No id column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let rows =
        File::get_files_by_folder_id(desktop_id, vec!["id", "file_name", "execute_path"], &pool)
            .await?;

    let files = File::try_from_vec(rows, None);

    let rows =
        Folder::get_children_folders(desktop_id, vec!["id", "folder_name", "folder_type"], &pool)
            .await?;

    let folders = Folder::try_from_vec(rows, None);

    Ok(render_screen_grid(form.height, form.width, desktop_id, files, folders).render())
}
