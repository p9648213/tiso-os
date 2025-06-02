use axum::{Form, response::Html};
use vy::IntoHtml;

use crate::views::{
    ItemType,
    screen_v::{render_screen, render_screen_grid, render_screen_item},
};

use super::GridForm;

pub async fn get_screen() -> Html<String> {
    Html(render_screen().into_string())
}

pub async fn create_screen_grid(Form(form): Form<GridForm>) -> Html<String> {
    Html(render_screen_grid(form.height, form.width).into_string())
}

pub async fn create_txt() -> Html<String> {
    Html(render_screen_item(ItemType::Text).into_string())
}

pub async fn create_folder() -> Html<String> {
    Html(render_screen_item(ItemType::Folder).into_string())
}
