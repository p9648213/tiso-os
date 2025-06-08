use axum::{Extension, Form, response::IntoResponse};
use hypertext::Renderable;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    views::screen_v::{render_screen, render_screen_grid, render_welcome_screen},
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

pub async fn create_screen_grid(Form(form): Form<GridForm>) -> impl IntoResponse {
    render_screen_grid(form.height, form.width).render()
}
