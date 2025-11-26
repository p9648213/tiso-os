use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "terminal_file.stpl")]
pub struct TerminalFile;

pub fn render_terminal_file() -> String {
    TerminalFile {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "terminal_window.stpl")]
pub struct TerminalWindow<'a> {
    pub username: &'a str,
    pub top: i32,
    pub left: i32,
    pub window_width: i32,
    pub window_height: i32,
}

pub fn render_terminal_window(username: &str, parent_height: i32, parent_width: i32) -> String {
    let window_width = parent_width * 40 / 100;
    let window_height = parent_height * 60 / 100;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    TerminalWindow {
        username,
        top,
        left,
        window_width,
        window_height,
    }
    .render_once()
    .unwrap()
}
