use axum::{Extension, extract::State, response::IntoResponse};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, file_db::File},
    utilities::user_utils::parse_user_id,
    views::taskbar_v::render_taskbar_menu_files,
};

pub async fn get_taskbar_menu_files(
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let _ = parse_user_id(user_id)?;

    let rows = File::get_taskbar_menu_files(vec!["file_type"], &pool).await?;
    let files = File::try_from_vec(rows, None);

    Ok(render_taskbar_menu_files(&files).render())
}
