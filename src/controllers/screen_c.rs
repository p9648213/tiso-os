use axum::{Form, response::IntoResponse};
use hypertext::Renderable;

use crate::views::{
    ItemType,
    screen_v::{render_screen, render_screen_grid, render_screen_item},
};

use super::GridForm;

pub async fn get_screen() -> impl IntoResponse {
    render_screen().render()
}

pub async fn create_screen_grid(Form(form): Form<GridForm>) -> impl IntoResponse {
    render_screen_grid(form.height, form.width).render()
}

pub async fn create_txt() -> impl IntoResponse {
    render_screen_item(ItemType::Text).render()
}

pub async fn create_folder() -> impl IntoResponse {
    render_screen_item(ItemType::Folder).render()
}
