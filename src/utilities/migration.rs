use deadpool_postgres::Pool;

use crate::utilities::postgres::excute;

pub async fn init_database(pool: &Pool) {
    let query = "CREATE TABLE IF NOT EXISTS users (
      id SERIAL PRIMARY KEY,
      username VARCHAR(255) UNIQUE NOT NULL,
      password VARCHAR(255) NOT NULL,
      created_at TIMESTAMPTZ DEFAULT NOW()
    )";

    excute(query, &[], pool).await.unwrap();
}
