use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        display_setting_db::{BackgroundType, DisplaySetting},
        error::AppError,
    },
    utilities::user_utils::parse_user_id,
    views::display_setting_v::render_display_setting_window,
};

pub async fn get_display_setting_window(
    Path((height, width)): Path<(i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let row = DisplaySetting::get_setting_by_user_id(
        user_id,
        vec!["background_type", "background_picture", "background_color"],
        &pool,
    )
    .await?;

    let display_setting = DisplaySetting::try_from(&row, None);

    let background_type = display_setting.background_type.ok_or_else(|| {
        tracing::error!("No background_type column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let background_picture = display_setting.background_picture;

    let background_color = display_setting.background_color;

    Ok(render_display_setting_window(
        height,
        width,
        background_type,
        background_picture,
        background_color,
    )
    .render())
}

pub async fn update_display_setting_background_type(
    Path((background_type,)): Path<(String,)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let background_type = match background_type.as_str() {
        "SolidColor" => BackgroundType::SolidColor,
        "Picture" => BackgroundType::Picture,
        _ => BackgroundType::SolidColor,
    };

    DisplaySetting::update_background_type_by_user_id(user_id, background_type, &pool).await?;

    Ok(())
}

pub async fn update_display_setting_background_color(
    Path((background_color,)): Path<(String,)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    DisplaySetting::update_background_color_by_user_id(user_id, background_color, &pool).await?;

    Ok(())
}
