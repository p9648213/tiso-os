use askama::Template;

#[derive(Template)]
#[template(path = "snake/snake_file.html")]
pub struct SnakeFile;

pub fn render_snake_file() -> String {
    SnakeFile {}.render().unwrap()
}

#[derive(Template)]
#[template(path = "snake/snake_window.html")]
pub struct SnakeWindow {
    pub top: i32,
    pub left: i32,
}

pub fn render_snake_window(parent_height: i32, parent_width: i32) -> String {
    let left = ((parent_width / 2) - (900 / 2)).max(0);
    let top = ((parent_height / 2) - (600 / 2)).max(0);

    SnakeWindow { top, left }.render().unwrap()
}
