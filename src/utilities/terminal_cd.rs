use crate::models::{folder_db::Folder, state::SessionMap};
use deadpool_postgres::Pool;

#[derive(Debug)]
pub struct Cd<'a> {
    pub current_dir: &'a str,
    pub args: &'a Vec<String>,
    pub pool: &'a Pool,
    pub user_id: i32,
    pub session_map: &'a SessionMap,
}

impl<'a> Cd<'a> {
    pub fn new(
        current_dir: &'a str,
        args: &'a Vec<String>,
        session_map: &'a SessionMap,
        user_id: i32,
        pool: &'a Pool,
    ) -> Self {
        Cd {
            current_dir,
            args,
            pool,
            user_id,
            session_map,
        }
    }

    pub async fn go_to_path(&self) -> Result<String, String> {
        let arg = if self.args.len() > 1 {
            self.args.join(" ")
        } else {
            self.args[0].trim().to_string()
        };

        match arg.as_str() {
            "~" => {
                let session_map = self.session_map.pin_owned();
                session_map.insert(format!("current-dir-{}", self.user_id), "/".to_string());
                Ok("/".to_string())
            }
            ".." => {
                let path = self.current_dir.split('/').collect::<Vec<&str>>();
                let path = path[..path.len() - 1].join("/");

                if path.is_empty() {
                    let session_map = self.session_map.pin_owned();
                    session_map.insert(format!("current-dir-{}", self.user_id), "/".to_string());
                    Ok("/".to_string())
                } else {
                    let result =
                        Folder::get_folder_by_path(&path, self.user_id, vec!["id"], self.pool)
                            .await;

                    match result {
                        Ok(_) => {
                            let session_map = self.session_map.pin_owned();
                            session_map
                                .insert(format!("current-dir-{}", self.user_id), path.clone());
                            Ok(path)
                        }
                        Err(err) => Err(format!("{}{}", "Path not found: ", err)),
                    }
                }
            }
            _ => {
                let path = if self.current_dir == "/" {
                    format!("{}{}", self.current_dir, arg)
                } else {
                    format!("{}/{}", self.current_dir, arg)
                };

                let result =
                    Folder::get_folder_by_path(&path, self.user_id, vec!["id"], self.pool).await;

                match result {
                    Ok(_) => {
                        let session_map = self.session_map.pin_owned();
                        session_map.insert(format!("current-dir-{}", self.user_id), path.clone());
                        Ok(path)
                    }
                    Err(err) => Err(format!("{}{}", "Path not found: ", err)),
                }
            }
        }
    }
}
