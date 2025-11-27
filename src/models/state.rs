use std::sync::Arc;

use axum::extract::FromRef;
use deadpool_postgres::Pool;
use papaya::HashMap;
use tokio::sync::Mutex;

pub type SessionMap = Arc<HashMap<String, String>>;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub session_map: SessionMap,
    pub pool: Pool,
    // A mutex to guard the file. We use a dummy type () because we just need the lock.
    pub file_lock: Arc<Mutex<()>>
}
