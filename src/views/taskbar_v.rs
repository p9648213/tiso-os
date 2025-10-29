use sailfish::TemplateSimple;

use crate::models::file_db::{File, FileType};

#[derive(TemplateSimple)]
#[template(path = "taskbar.stpl")]
struct Taskbar {}

pub fn render_taskbar() -> String {
    Taskbar {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "taskbar_menu_files.stpl")]
pub struct TaskbarMenuFiles<'a> {
    pub files: &'a Vec<File>,
}

pub fn render_taskbar_menu_files(files: &Vec<File>) -> String {
    TaskbarMenuFiles { files }.render_once().unwrap()
}
