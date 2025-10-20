use std::vec;

use axum::{
    Extension, Form,
    extract::State,
    response::{Html, IntoResponse},
};
use base64::{Engine, engine::general_purpose};
use deadpool_postgres::Pool;
use hypertext::Renderable;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        display_setting_db::{BackgroundType, DisplaySetting},
        error::AppError,
        folder_db::Folder,
        folder_item::FolderItem,
    },
    utilities::general::parse_user_id,
    views::{
        screen_v::render_screen_grid,
        screen_v_2::{render_main_screen, render_welcome_screen},
    },
};

#[derive(Deserialize)]
pub struct GridForm {
    pub height: u16,
    pub width: u16,
}

pub async fn get_screen(
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    if user_id.0.is_none() {
        return Ok(Html(render_welcome_screen()));
    }

    let user_id = parse_user_id(user_id)?;

    let display_setting = DisplaySetting::get_setting_by_user_id(
        user_id,
        vec![
            "background_type",
            "background_picture",
            "background_color",
            "background_content_type",
        ],
        &pool,
    )
    .await?;

    let background = match display_setting.background_type.unwrap() {
        BackgroundType::SolidColor => display_setting.background_color.unwrap_or_default(),
        BackgroundType::Picture => {
            format!(
                "url('data:{};base64,{}');",
                display_setting.background_content_type.unwrap(),
                general_purpose::STANDARD
                    .encode(display_setting.background_picture.unwrap_or_default())
            )
        }
    };

    Ok(Html(render_main_screen(&background)))
}

pub async fn create_screen_grid(
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    Form(form): Form<GridForm>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let desktop_folder =
        Folder::get_desktop_folder(user_id, vec!["id", "sort_type"], &pool).await?;

    let desktop_id = desktop_folder.id.unwrap();
    let sort_type = desktop_folder.sort_type.unwrap();

    let desktop_items =
        FolderItem::get_desktop_items(desktop_id, user_id, &sort_type, &pool).await?;

    Ok(render_screen_grid(
        form.height,
        form.width,
        desktop_id,
        &sort_type,
        desktop_items,
    )
    .render())
}
