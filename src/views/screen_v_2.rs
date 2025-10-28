use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "screen/welcome_screen.stpl")]
struct WelcomeScreen {}

pub fn render_welcome_screen() -> String {
    WelcomeScreen {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "screen/confirm_password.stpl")]
struct ConfirmPassword {
    confirm_password: Option<String>,
    register_mode: bool,
}

pub fn render_confirm_password(value: Option<String>, register_mode: bool) -> String {
    ConfirmPassword {
        confirm_password: value,
        register_mode,
    }
    .render_once()
    .unwrap()
}
