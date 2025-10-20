use axum::{
    Form,
    body::Body,
    extract::State,
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use deadpool_postgres::Pool;
use rand::Rng;
use serde::Deserialize;

use crate::{
    models::{error::AppError, state::SessionMap, user_db::User},
    utilities::argon::{compare_password, hash_password},
    views::screen_v::{render_confirm_password, render_screen_section},
};

#[derive(Deserialize, Debug, Clone)]
pub struct AccountForm {
    pub username: String,
    pub password: String,
    pub confirm_password: Option<String>,
}

pub async fn create_account(
    header: HeaderMap,
    State(pool): State<Pool>,
    State(session_map): State<SessionMap>,
    Form(account_form): Form<AccountForm>,
) -> Result<impl IntoResponse, AppError> {
    let hx_trigger = header
        .get("HX-Trigger")
        .ok_or_else(|| {
            tracing::error!("Failed to get HX-Trigger header");
            AppError::new(StatusCode::BAD_REQUEST, "Bad Request")
        })?
        .to_str()
        .map_err(|err| {
            tracing::error!("Failed to convert HX-Trigger header to str: {:?}", err);
            AppError::new(StatusCode::BAD_REQUEST, "Bad Request")
        })?;

    match hx_trigger {
        "account_username" => {
            let row = User::get_user_by_username(&account_form.username, vec!["id"], &pool).await?;

            if row.is_some() {
                Ok(render_confirm_password(account_form.confirm_password, false).into_response())
            } else {
                Ok(render_confirm_password(account_form.confirm_password, true).into_response())
            }
        }
        "account_confirm_password" => {
            let password = account_form.password;
            let confirm_password = account_form.confirm_password.unwrap_or_default();

            if password == confirm_password {
                Ok("".into_response())
            } else {
                Ok("Passwords do not match".into_response())
            }
        }
        "account_form" => {
            if account_form.username.is_empty() || account_form.password.is_empty() {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("HX-Retarget", "#account_error")
                    .body(Body::new("Input fields cannot be empty".to_string()))
                    .map_err(|err| {
                        tracing::error!("Failed to build response: {:?}", err);
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                    })?;

                return Ok(response);
            }

            let row =
                User::get_user_by_username(&account_form.username, vec!["id", "password"], &pool)
                    .await?;

            if let Some(row) = row {
                let user = User::try_from(&row, None);

                let user_password = user.password.ok_or_else(|| {
                    tracing::error!("No password column or value is null");
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                })?;

                if compare_password(&account_form.password, &user_password)? {
                    let user_id = user.id.ok_or_else(|| {
                        tracing::error!("No id column or value is null");
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                    })?;

                    let session: i128 = rand::rng().random();

                    session_map
                        .pin()
                        .insert(session.to_string(), user_id.to_string());

                    let cookie = Cookie::build(("session", session.to_string()))
                        .http_only(true)
                        .same_site(SameSite::Lax)
                        .permanent()
                        .path("/")
                        .secure(false);

                    let cookie_jar = CookieJar::new().add(cookie);

                    Ok((cookie_jar, render_screen_section()).into_response())
                } else {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header("HX-Retarget", "#account_error")
                        .body(Body::new("Invalid password".to_string()))
                        .map_err(|err| {
                            tracing::error!("Failed to build response: {:?}", err);
                            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                        })?;

                    Ok(response)
                }
            } else {
                let password = account_form.password;
                let confirm_password = account_form.confirm_password.unwrap_or_default();

                if password == confirm_password {
                    let user_id = User::create_user(
                        &account_form.username,
                        &hash_password(&password)?,
                        &pool,
                    )
                    .await?;

                    let session: i128 = rand::rng().random();

                    session_map
                        .pin()
                        .insert(session.to_string(), user_id.to_string());

                    let cookie = Cookie::build(("session", session.to_string()))
                        .http_only(true)
                        .same_site(SameSite::Lax)
                        .permanent()
                        .path("/")
                        .secure(false);

                    let cookie_jar = CookieJar::new().add(cookie);

                    Ok((cookie_jar, render_screen_section()).into_response())
                } else {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header("HX-Retarget", "#account_error")
                        .body(Body::new("Passwords do not match".to_string()))
                        .map_err(|err| {
                            tracing::error!("Failed to build response: {:?}", err);
                            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
                        })?;

                    Ok(response)
                }
            }
        }
        _ => Err(AppError::new(StatusCode::BAD_REQUEST, "Bad Request")),
    }
}
