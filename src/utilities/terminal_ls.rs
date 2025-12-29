use deadpool_postgres::Pool;

use crate::{
    models::{
        folder_db::{Folder, FolderSortType},
        folder_item::{FolderItem, ItemType},
    },
    views::terminal_v::{ItemDetail, render_terminal_ls},
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
                Folder::get_folders_with_no_parent(self.user_id, vec!["folder_name", "created_at"], self.pool)
                    .await;

            match result {
                Ok(folder_names) => {
                    let item_details: Vec<ItemDetail> = folder_names
                        .into_iter()
                        .map(|folder| ItemDetail {
                            item_name: folder.folder_name.unwrap(),
                            item_type: ItemType::Folder,
                            created_at: folder.created_at.unwrap().to_string(),
                        })
                        .collect();

                    render_terminal_ls(item_details)
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
                            let item_details: Vec<ItemDetail> = items
                                .into_iter()
                                .map(|item| ItemDetail {
                                    item_name: item.name.unwrap(),
                                    item_type: item.item_type.unwrap(),
                                    created_at: item.created_at.unwrap().to_string(),
                                })
                                .collect();

                            render_terminal_ls(item_details)
                        }
                        Err(err) => err.to_string(),
                    }
                }
                Err(err) => err.to_string(),
            }
        }
    }
}
