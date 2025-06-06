use axum::{Extension, Form, response::IntoResponse};
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId,
    views::{
        ItemType,
        screen_v::{render_screen, render_screen_grid, render_screen_item, render_welcome_screen},
    },
};

use super::GridForm;

pub async fn get_screen(Extension(user_id): Extension<UserId>) -> impl IntoResponse {
    if user_id.0.is_none() {
        return render_welcome_screen().render();
    }
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
