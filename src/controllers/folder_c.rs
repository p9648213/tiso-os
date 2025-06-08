use axum::response::IntoResponse;
use hypertext::Renderable;

use crate::views::screen_v::{ItemType, render_screen_item};

pub async fn create_folder() -> impl IntoResponse {
    render_screen_item(ItemType::Folder).render()
}
