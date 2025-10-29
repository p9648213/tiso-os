use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "snake/snake_file.stpl")]
pub struct SnakeFile;

pub fn render_snake_file() -> String {
    SnakeFile {}.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "snake/snake_window.stpl")]
pub struct SnakeWindow {
    pub top: i32,
    pub left: i32,
}

pub fn render_snake_window(parent_height: i32, parent_width: i32) -> String {
    let left = ((parent_width / 2) - (900 / 2)).max(0);
    let top = ((parent_height / 2) - (600 / 2)).max(0);

    SnakeWindow { top, left }.render_once().unwrap()
}
