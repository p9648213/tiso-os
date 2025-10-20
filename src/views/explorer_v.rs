use crate::models::{
    file_db::FileType,
    folder_item::{FolderItem, ItemType},
};
use askama::Template;

#[derive(Template)]
#[template(path = "explorer/explorer_window.html")]
pub struct ExplorerWindow<'a> {
    pub folder_id: i32,
    pub folder_name: &'a str,
    pub window_width: i32,
    pub window_height: i32,
    pub left: i32,
    pub top: i32,
    pub folder_items: &'a Vec<FolderItem>,
}

pub fn render_explorer_window(
    folder_id: i32,
    folder_name: String,
    parent_width: i32,
    parent_height: i32,
    folder_items: &Vec<FolderItem>,
) -> String {
    let window_width = parent_width * 70 / 100;
    let window_height = parent_height * 90 / 100;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    ExplorerWindow {
        folder_id,
        folder_name: &folder_name,
        window_width,
        window_height,
        left,
        top,
        folder_items,
    }
    .render()
    .unwrap()
}
