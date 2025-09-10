use axum::{
    Extension,
    extract::{Path, State},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId, models::error::AppError, utilities::user_utils::parse_user_id,
    views::display_setting_v::render_display_setting_window,
};

pub async fn get_display_setting_window(
    Path((height, width)): Path<(i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let _user_id = parse_user_id(user_id)?;

    Ok(render_display_setting_window(height, width).render())
}
