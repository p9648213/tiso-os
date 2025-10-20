use askama::Template;

use crate::models::file_db::{File, FileType};

#[derive(Template)]
#[template(path = "taskbar/taskbar.html")]
struct Taskbar {}

pub fn render_taskbar() -> String {
    Taskbar {}.render().unwrap()
}

#[derive(Template)]
#[template(path = "taskbar/taskbar_menu_files.html")]
pub struct TaskbarMenuFiles<'a> {
    pub files: &'a Vec<File>,
}

pub fn render_taskbar_menu_files(files: &Vec<File>) -> String {
    TaskbarMenuFiles { files }.render().unwrap()
}
