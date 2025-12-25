use std::f32::consts::E;

use deadpool_postgres::Pool;

use crate::models::folder_db::{Folder, FolderType};

pub struct Mkdir<'a> {
    pub current_dir: &'a str,
    pub args: &'a Vec<String>,
    pub pool: &'a Pool,
    pub user_id: i32,
}

impl<'a> Mkdir<'a> {
    pub fn new(current_dir: &'a str, args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Mkdir {
            current_dir,
            args,
            pool,
            user_id,
        }
    }

    pub async fn create_folder(&self) -> Result<String, String> {
        println!("{:?}", self.args);

        if self.current_dir == "/" {
            return Err(r#"Cannot create folder in path "/". Use "cd" to change directory."#.to_string())
        }

        let file_name = self.args.join(" ");

        let file_path = if self.current_dir == "/" {
            format!("{}{}", self.current_dir, file_name)
        } else {
            format!("{}/{}", self.current_dir, file_name)
        };

        let current_folder =
            Folder::get_folder_by_path(self.current_dir, self.user_id, vec!["id", "folder_type"], self.pool)
                .await
                .map_err(|err| err.to_string())?;

        if current_folder.folder_type.unwrap() == FolderType::Desktop {
            // TODO: Implement
            Ok("Created".to_string())
        } else {
            // let folder = Folder::create_folder(
            //     self.user_id,
            //     file_name,
            //     FolderType::Normal,
            //     Some(current_folder.id.unwrap()),
            //     None,
            //     file_path,
            //     self.pool,
            // )
            // .await.map_err(|err| err.to_string())?;

            // Ok(format!("{} created", folder.folder_name.unwrap()))

            Ok("Created".to_string())
        }
    }
}
