use std::sync::Arc;

use axum::extract::FromRef;
use deadpool_postgres::Pool;
use papaya::HashMap;

pub type SessionMap = Arc<HashMap<String, String>>;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub session_map: SessionMap,
    pub pool: Pool,
}
