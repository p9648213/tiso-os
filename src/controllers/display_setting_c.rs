use axum::{
    Extension,
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{Engine, engine::general_purpose};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    constant::MAX_BACKGROUND_PICTURE_SIZE,
    middlewares::session_mw::UserId,
    models::{
        display_setting_db::{BackgroundType, DisplaySetting},
        error::AppError,
    },
    utilities::general::parse_user_id,
    views::{display_setting_v::render_display_setting_window, screen_v::render_screen_background},
};

pub async fn get_display_setting_window(
    Path((height, width)): Path<(i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let display_setting = DisplaySetting::get_setting_by_user_id(
        user_id,
        vec!["background_type", "background_color"],
        &pool,
    )
    .await?;

    Ok((
        [(
            "HX-Trigger",
            r#"{"openFile":{"image":"/assets/images/display-setting.svg", "window_id": "display-setting-window"}}"#,
        )],
        render_display_setting_window(
            height,
            width,
            display_setting.background_type.unwrap(),
            display_setting.background_color,
        )
        .render(),
    ))
}

pub async fn update_background_type(
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

    DisplaySetting::update_background_type_by_user_id(user_id, &background_type, &pool).await?;

    let background = match background_type {
        BackgroundType::SolidColor => {
            let display_setting =
                DisplaySetting::get_setting_by_user_id(user_id, vec!["background_color"], &pool)
                    .await?;

            display_setting.background_color.unwrap_or_default()
        }
        BackgroundType::Picture => {
            let display_setting = DisplaySetting::get_setting_by_user_id(
                user_id,
                vec!["background_picture", "background_content_type"],
                &pool,
            )
            .await?;

            let content_type = display_setting.background_content_type.unwrap_or_default();

            format!(
                "url('data:{};base64,{}');",
                content_type,
                general_purpose::STANDARD
                    .encode(display_setting.background_picture.unwrap_or_default())
            )
        }
    };

    Ok(render_screen_background(&background).render())
}

pub async fn update_background_color(
    Path((background_color,)): Path<(String,)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    DisplaySetting::update_background_color_by_user_id(user_id, background_color, &pool).await?;

    Ok(())
}

pub async fn upload_background_picture(
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let mut content_type = String::new();
    let mut file_bytes = Bytes::new();

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!("Error while reading multipart field: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })? {
        content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_default();

        if !content_type.starts_with("image/") {
            tracing::error!("Invalid content type: {}", content_type);
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Invalid content type",
            ));
        }

        file_bytes = field.bytes().await.map_err(|err| {
            tracing::error!("Error while reading multipart field: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        if file_bytes.len() > MAX_BACKGROUND_PICTURE_SIZE {
            tracing::error!("File size is too large: {:?}", file_bytes);
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "File size is too large",
            ));
        }

        if file_bytes.is_empty() {
            tracing::error!("File is empty");
            return Err(AppError::new(StatusCode::BAD_REQUEST, "File is empty"));
        }

        DisplaySetting::update_background_picture_by_user_id(
            user_id,
            Some(file_bytes.to_vec()),
            &content_type,
            &pool,
        )
        .await?;
    }

    Ok(render_screen_background(&format!(
        "url('data:{};base64,{}');",
        content_type,
        general_purpose::STANDARD.encode(file_bytes)
    ))
    .render())
}
