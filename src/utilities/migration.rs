use deadpool_postgres::Pool;

use crate::utilities::postgres::{excute, query_one};

pub async fn init_database(pool: &Pool) {
    let sql = "CREATE TABLE IF NOT EXISTS users (
      id SERIAL PRIMARY KEY,
      username VARCHAR(255) UNIQUE NOT NULL,
      password VARCHAR(255) NOT NULL,
      created_at TIMESTAMPTZ DEFAULT NOW()
    );";

    excute(sql, &[], pool).await.unwrap();

    let sql = "DO $$
      BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'foldertype') THEN
          CREATE TYPE FolderType AS ENUM ('Normal', 'Root', 'Desktop');
        END IF;
      END
    $$;";

    excute(sql, &[], pool).await.unwrap();

    let sql = "CREATE TABLE IF NOT EXISTS folders (
      id SERIAL PRIMARY KEY,
      user_id INT,
      folder_name VARCHAR(255) NOT NULL,
      folder_type FolderType NOT NULL,
      parent_folder_id INT,
      created_at TIMESTAMPTZ DEFAULT NOW(),
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
      FOREIGN KEY (parent_folder_id) REFERENCES folders(id) ON DELETE CASCADE
    );";

    excute(sql, &[], pool).await.unwrap();

    let sql = "CREATE TABLE IF NOT EXISTS files (
      id SERIAL PRIMARY KEY,
      user_id INT,
      folder_id INT,
      file_name VARCHAR(255) NOT NULL,
      execute_path VARCHAR(255) NOT NULL,
      created_at TIMESTAMPTZ DEFAULT NOW(),
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
      FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE
    );";

    excute(sql, &[], pool).await.unwrap();

    let sql = "SELECT * from files WHERE file_name = 'Calculator'";

    if query_one(sql, &[], pool).await.is_err() {
        let sql = "INSERT INTO files (file_name, execute_path) VALUES ('Calculator', '/execute/calculator');";
        excute(sql, &[], pool).await.unwrap();
    }
}
