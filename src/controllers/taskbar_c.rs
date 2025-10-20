use axum::{Extension, extract::State, response::IntoResponse};
use deadpool_postgres::Pool;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, file_db::File},
    utilities::general::parse_user_id,
    views::taskbar_v::render_taskbar_menu_files,
};

pub async fn get_taskbar_menu_files(
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let files = File::get_taskbar_menu_files(user_id, vec!["id", "file_type"], &pool).await?;

    Ok(render_taskbar_menu_files(&files))
}
