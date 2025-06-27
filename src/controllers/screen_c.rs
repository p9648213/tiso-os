use axum::{Extension, Form, extract::State, http::StatusCode, response::IntoResponse};
use deadpool_postgres::Pool;
use hypertext::Renderable;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId, models::{desktop::DesktopItem, error::AppError, folders_db::Folder}, utilities::user_utils::parse_user_id, views::screen_v::{render_screen, render_screen_grid, render_welcome_screen}
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
    let user_id = parse_user_id(user_id)?;

    let rows = Folder::get_desktop_folders(user_id, vec!["id", "sort_type"], &pool).await?;

    let desktop_folder = Folder::try_from(&rows, None);

    let desktop_id = desktop_folder.id.ok_or_else(|| {
        tracing::error!("No id column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let sort_type = desktop_folder.sort_type.ok_or_else(|| {
        tracing::error!("No sort_type column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let rows = DesktopItem::get_desktop_items(desktop_id, &sort_type, &pool).await?;

    let items = DesktopItem::try_from_vec(rows, None);

    Ok(render_screen_grid(form.height, form.width, desktop_id, &sort_type, items).render())
}
