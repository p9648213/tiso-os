use axum::{extract::Path, response::IntoResponse};
use hypertext::Renderable;

use crate::views::flappy_bird_v::render_flappy_bird_window;

pub async fn get_flappy_bird_window(Path((height, width)): Path<(i32, i32)>) -> impl IntoResponse {
    render_flappy_bird_window(height, width).render()
}
