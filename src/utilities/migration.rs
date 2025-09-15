use deadpool_postgres::Pool;

use crate::utilities::postgres::DbExecutor;

pub async fn init_database(pool: &Pool) {
    let client = pool.get().await.unwrap();

    let sql = r#"CREATE TABLE IF NOT EXISTS "user" (
      id SERIAL PRIMARY KEY,
      username VARCHAR(255) UNIQUE NOT NULL,
      password VARCHAR(255) NOT NULL,
      created_at TIMESTAMPTZ DEFAULT NOW()
    );"#;

    client.execute(sql, &[]).await.unwrap();

    let sql = "DO $$
      BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'foldertype') THEN
          CREATE TYPE FolderType AS ENUM ('Normal', 'Root', 'Desktop');
        END IF;
      END
    $$;";

    client.execute(sql, &[]).await.unwrap();

    let sql = "DO $$
      BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'foldersorttype') THEN
          CREATE TYPE FolderSortType AS ENUM ('Custom', 'DateCreated');
        END IF;
      END
    $$;";

    client.execute(sql, &[]).await.unwrap();

    let sql = r#"CREATE TABLE IF NOT EXISTS folder (
      id SERIAL PRIMARY KEY,
      user_id INT,
      parent_folder_id INT,
      folder_name VARCHAR(255) NOT NULL,
      folder_type FolderType NOT NULL,
      sort_type FolderSortType NOT NULL DEFAULT 'DateCreated',
      desktop_position VARCHAR(32),
      created_at TIMESTAMPTZ DEFAULT NOW(),
      FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
      FOREIGN KEY (parent_folder_id) REFERENCES folder(id) ON DELETE CASCADE
    );"#;

    client.execute(sql, &[]).await.unwrap();

    let sql = "DO $$
      BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'filetype') THEN
          CREATE TYPE FileType AS ENUM ('Txt', 'Calculator', 'Snake', 'FlappyBird');
        END IF;
      END
    $$;";

    client.execute(sql, &[]).await.unwrap();

    let sql = r#"CREATE TABLE IF NOT EXISTS file (
      id SERIAL PRIMARY KEY,
      user_id INT,
      folder_id INT,
      file_name VARCHAR(255) NOT NULL,
      file_type FileType NOT NULL,
      desktop_position VARCHAR(32),
      created_at TIMESTAMPTZ DEFAULT NOW(),
      FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
      FOREIGN KEY (folder_id) REFERENCES folder(id) ON DELETE CASCADE
    );"#;

    client.execute(sql, &[]).await.unwrap();

    let sql = "CREATE TABLE IF NOT EXISTS txt (
      id SERIAL PRIMARY KEY,
      file_id INT UNIQUE,
      FOREIGN KEY (file_id) REFERENCES file(id) ON DELETE CASCADE
    );";

    client.execute(sql, &[]).await.unwrap();

    let sql = "CREATE TABLE IF NOT EXISTS calculator (
      id SERIAL PRIMARY KEY,
      file_id INT UNIQUE,
      FOREIGN KEY (file_id) REFERENCES file(id) ON DELETE CASCADE
    );";

    client.execute(sql, &[]).await.unwrap();

    let sql = "SELECT * from file WHERE file_name = 'Calculator'";

    if client.query(sql, &[]).await.unwrap().is_empty() {
        let sql = "INSERT INTO file (file_name, file_type) VALUES ('Calculator', 'Calculator') RETURNING id;";
        let id: i32 = client.query_one(sql, &[]).await.unwrap().get("id");

        let sql = "INSERT INTO calculator (file_id) VALUES ($1);";
        client.execute(sql, &[&id]).await.unwrap();
    }

    let sql = "CREATE TABLE IF NOT EXISTS snake (
      id SERIAL PRIMARY KEY,
      file_id INT UNIQUE,
      FOREIGN KEY (file_id) REFERENCES file(id) ON DELETE CASCADE
    );";

    client.execute(sql, &[]).await.unwrap();

    let sql = "SELECT * from file WHERE file_name = 'Snake'";

    if client.query(sql, &[]).await.unwrap().is_empty() {
        let sql = "INSERT INTO file (file_name, file_type) VALUES ('Snake', 'Snake') RETURNING id;";
        let id: i32 = client.query_one(sql, &[]).await.unwrap().get("id");

        let sql = "INSERT INTO snake (file_id) VALUES ($1);";
        client.execute(sql, &[&id]).await.unwrap();
    }

    let sql = "CREATE TABLE IF NOT EXISTS flappybird (
      id SERIAL PRIMARY KEY,
      file_id INT UNIQUE,
      FOREIGN KEY (file_id) REFERENCES file(id) ON DELETE CASCADE
    );";

    client.execute(sql, &[]).await.unwrap();

    let sql = "SELECT * from file WHERE file_name = 'FlappyBird'";

    if client.query(sql, &[]).await.unwrap().is_empty() {
        let sql = "INSERT INTO file (file_name, file_type) VALUES ('FlappyBird', 'FlappyBird') RETURNING id;";
        let id: i32 = client.query_one(sql, &[]).await.unwrap().get("id");

        let sql = "INSERT INTO flappybird (file_id) VALUES ($1);";
        client.execute(sql, &[&id]).await.unwrap();
    }

    let sql = "DO $$
      BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'backgroundtype') THEN
          CREATE TYPE BackgroundType AS ENUM ('SolidColor', 'Picture');
        END IF;
      END
    $$;";

    client.execute(sql, &[]).await.unwrap();

    let sql = r#"CREATE TABLE IF NOT EXISTS display_setting (
      id SERIAL PRIMARY KEY,
      user_id INT,
      background_type BackgroundType NOT NULL DEFAULT 'SolidColor',
      background_picture BYTEA,
      background_color VARCHAR(255) NOT NULL DEFAULT 'radial-gradient(ellipse at top left, #070f2b, #1b1a55, #535c91)',
      background_content_type VARCHAR(255) NOT NULL DEFAULT 'image/png',
      created_at TIMESTAMPTZ DEFAULT NOW(),
      FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE
    );"#;

    client.execute(sql, &[]).await.unwrap();
}
