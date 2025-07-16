use axum::response::IntoResponse;
use hypertext::Renderable;

use crate::views::snake_v::render_snake_window;

pub async fn get_snake_window() -> impl IntoResponse {
    render_snake_window().render()
}
