use askama::Template;

#[derive(Template)]
#[template(path = "taskbar/taskbar.html")]
struct Taskbar {}

pub fn render_taskbar() -> String {
    Taskbar {}.render().unwrap()
}
