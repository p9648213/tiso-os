use deadpool_postgres::Pool;

use crate::{
    models::{
        folder_db::{Folder, FolderSortType},
        folder_item::FolderItem,
    },
    views::terminal_v::render_terminal_ls,
};

pub struct Ls<'a> {
    pub current_dir: String,
    pub user_id: i32,
    pub pool: &'a Pool,
}

impl<'a> Ls<'a> {
    pub fn new(current_dir: &str, user_id: i32, pool: &'a Pool) -> Self {
        Ls {
            current_dir: current_dir.to_string(),
            user_id,
            pool,
        }
    }

    pub async fn list_file(&self) -> String {
        if self.current_dir == "/" {
            let result =
                Folder::get_folders_with_no_parent(self.user_id, vec!["folder_name"], self.pool)
                    .await;

            match result {
                Ok(folder_names) => {
                    let folder_name: Vec<String> = folder_names
                        .into_iter()
                        .map(|folder| folder.folder_name.unwrap_or_default())
                        .collect();

                    render_terminal_ls(folder_name)
                }
                Err(err) => err.to_string(),
            }
        } else {
            let result =
                Folder::get_folder_by_path(&self.current_dir, self.user_id, vec!["id"], self.pool)
                    .await;

            match result {
                Ok(folder) => {
                    let result = FolderItem::get_folder_items(
                        folder.id.unwrap(),
                        self.user_id,
                        &FolderSortType::DateCreated,
                        self.pool,
                    )
                    .await;

                    match result {
                        Ok(items) => {
                            let item_names: Vec<String> = items
                                .into_iter()
                                .map(|item| item.name.unwrap_or_default())
                                .collect();

                            render_terminal_ls(item_names)
                        },
                        Err(err) => err.to_string(),
                    }
                }
                Err(err) => err.to_string(),
            }
        }
    }
}
