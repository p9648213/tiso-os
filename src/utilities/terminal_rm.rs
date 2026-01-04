use deadpool_postgres::Pool;

use crate::models::{
    file_db::File,
    folder_db::{Folder, FolderSortType},
    folder_item::{FolderItem, ItemType},
};

pub struct Rm<'a> {
    pub current_dir: String,
    pub args: &'a Vec<String>,
    pub user_id: i32,
    pub pool: &'a Pool,
}

impl<'a> Rm<'a> {
    pub fn new(current_dir: &str, args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Rm {
            current_dir: current_dir.to_string(),
            args,
            user_id,
            pool,
        }
    }

    pub async fn remove_item(&self) -> Result<String, String> {
        let item_name = if self.args.len() > 1 {
            self.args.join(" ")
        } else {
            self.args[0].trim().to_string()
        };

        if self.current_dir == "/" {
            Err(r#"Cannot remove folder in path "/". Use "cd" to change directory."#.to_string())
        } else {
            let current_folder =
                Folder::get_folder_by_path(&self.current_dir, self.user_id, vec!["id"], self.pool)
                    .await
                    .map_err(|err| err.to_string())?;

            let folder_items = FolderItem::get_folder_items(
                current_folder.id.unwrap(),
                self.user_id,
                &FolderSortType::Custom,
                self.pool,
            )
            .await
            .map_err(|err| err.to_string())?;

            let deleted_item = folder_items
                .iter()
                .find(|item| item.name.as_ref().unwrap() == &item_name);

            if let Some(deleted_item) = deleted_item {
                match deleted_item.item_type.as_ref().unwrap() {
                    ItemType::File => {
                        File::delete_file(deleted_item.id.unwrap(), self.user_id, self.pool)
                            .await
                            .map_err(|err| err.to_string())?;
                    }
                    ItemType::Folder => {
                        Folder::delete_folder(deleted_item.id.unwrap(), self.user_id, self.pool)
                            .await
                            .map_err(|err| err.to_string())?;
                    }
                }
            } else {
                return Err(format!("Not found: {}", item_name));
            }

            Ok(format!("{} removed", item_name))
        }
    }
}
