use askama::Template;

#[derive(Template)]
#[template(path = "flappybird/flappy_bird_file.html")]
pub struct FlappyBirdFile;

pub fn render_flappy_bird_file() -> String {
    FlappyBirdFile {}.render().unwrap()
}

#[derive(Template)]
#[template(path = "flappybird/flappy_bird_window.html")]
pub struct FlappyBirdWindow {
    pub top: i32,
    pub left: i32,
}

pub fn render_flappy_bird_window(parent_height: i32, parent_width: i32) -> String {
    let left = ((parent_width / 2) - (800 / 2)).max(0);
    let top = ((parent_height / 2) - (512 / 2)).max(0);

    FlappyBirdWindow { top, left }.render().unwrap()
}
