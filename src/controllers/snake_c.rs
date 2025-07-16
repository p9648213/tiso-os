use axum::{extract::Path, response::IntoResponse};
use hypertext::Renderable;

use crate::views::snake_v::render_snake_window;

pub async fn get_snake_window(Path((height, width)): Path<(i32, i32)>) -> impl IntoResponse {
    render_snake_window(height, width).render()
}
