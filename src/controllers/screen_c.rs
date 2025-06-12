use axum::{Extension, Form, extract::State, http::StatusCode, response::IntoResponse};
use deadpool_postgres::Pool;
use hypertext::Renderable;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, folders_db::Folder},
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
    Form(form): Form<GridForm>,
    Extension(user_id): Extension<UserId>,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let rows =
        Folder::get_desktop_folders(user_id, vec!["folder_name", "folder_type"], &pool).await?;

    let desktop_folders = Folder::try_from(&rows, None);

    Ok(render_screen_grid(form.height, form.width, desktop_folders).render())
}
