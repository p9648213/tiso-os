use deadpool_postgres::Pool;

use crate::models::folder_db::Folder;

pub struct Cd<'a> {
    pub current_dir: &'a str,
    pub args: &'a Vec<String>,
    pub pool: &'a Pool,
    pub user_id: i32,
}

impl<'a> Cd<'a> {
    pub fn new(current_dir: &'a str, args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Cd {
            current_dir,
            args,
            pool,
            user_id,
        }
    }

    pub async fn go_to_path(&self) -> Result<String, String> {
        if self.args.len() > 1 || self.args.is_empty() {
            Err("Invalid arguments. Type help cd for more information.".to_string())
        } else {
            let path = format!("{}{}", self.current_dir, self.args[0].trim());
            let result =
                Folder::get_folder_by_path(&path, self.user_id, vec!["id"], self.pool).await;

            match result {
                Ok(_) => Ok(path.into()),
                Err(err) => Err(err.to_string()),
            }
        }
    }
}
