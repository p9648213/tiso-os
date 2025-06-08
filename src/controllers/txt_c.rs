use axum::response::IntoResponse;
use hypertext::Renderable;

use crate::views::screen_v::{ItemType, render_screen_item};

pub async fn create_txt() -> impl IntoResponse {
    render_screen_item(ItemType::Text).render()
}
