use axum::{Form, response::Html};
use hypertext::Renderable;
use vy::IntoHtml;

use crate::views::{
    screen_v::{render_screen, render_screen_grid, render_screen_grid_2, render_screen_grid_3, render_screen_grid_4, render_screen_item}, ItemType
};

use super::GridForm;

pub async fn get_screen() -> Html<String> {
    Html(render_screen().into_string())
}

pub async fn create_screen_grid(Form(form): Form<GridForm>) -> Html<String> {
    Html(render_screen_grid_3(form.height, form.width).render().0)
}

pub async fn create_txt() -> Html<String> {
    Html(render_screen_item(ItemType::Text).into_string())
}

pub async fn create_folder() -> Html<String> {
    Html(render_screen_item(ItemType::Folder).into_string())
}
