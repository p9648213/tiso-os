use askama::Template;

#[derive(Template)]
#[template(path = "screen/welcome_screen.html")]
struct WelcomeScreen {
    confirm_password: Option<String>,
    register_mode: bool,
}

pub fn render_welcome_screen() -> String {
    WelcomeScreen {
        confirm_password: None,
        register_mode: true,
    }
    .render()
    .unwrap()
}

#[derive(Template)]
#[template(path = "screen/confirm_password.html")]
struct ConfirmPassword {
    confirm_password: Option<String>,
    register_mode: bool,
}

pub fn render_confirm_password(value: Option<String>, register_mode: bool) -> String {
    ConfirmPassword {
        confirm_password: value,
        register_mode,
    }
    .render()
    .unwrap()
}

#[derive(Template)]
#[template(path = "screen/main_screen.html")]
struct MainScreen<'a> {
    background: &'a str,
}

pub fn render_main_screen(background: &str) -> String {
    MainScreen { background }.render().unwrap()
}

#[derive(Template)]
#[template(path = "screen/screen_background.html")]
struct ScreenBackground<'a> {
    background: &'a str,
}

pub fn render_screen_background(background: &str) -> String {
    ScreenBackground { background }.render().unwrap()
}

#[derive(Template)]
#[template(path = "screen/screen_section.html")]
struct ScreenSection {}

pub fn render_screen_section() -> String {
    ScreenSection {}.render().unwrap()
}
