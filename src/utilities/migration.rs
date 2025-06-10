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

    let query = "CREATE TABLE IF NOT EXISTS folders (
      id SERIAL PRIMARY KEY,
      user_id INT NOT NULL,
      folder_name VARCHAR(255) NOT NULL,
      parent_folder_id INT,
      created_at TIMESTAMPTZ DEFAULT NOW(),
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
      FOREIGN KEY (parent_folder_id) REFERENCES folders(id) ON DELETE CASCADE
    );";

    excute(query, &[], pool).await.unwrap();

    let query = "CREATE TABLE IF NOT EXISTS files (
      id SERIAL PRIMARY KEY,
      user_id INT NOT NULL,
      folder_id INT,
      file_name VARCHAR(255) NOT NULL,
      file_type VARCHAR(255) NOT NULL,
      created_at TIMESTAMPTZ DEFAULT NOW(),
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
      FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE
    );";

    excute(query, &[], pool).await.unwrap();
}
