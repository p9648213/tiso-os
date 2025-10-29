use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "calculator/calculator_file.stpl")]
pub struct CalculatorFile;

pub fn render_calculator_file() -> String {
    CalculatorFile {}.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "calculator/calculator_window.stpl")]
pub struct CalculatorWindow {
    pub top: i32,
    pub left: i32,
    pub window_width: i32,
    pub window_height: i32,
}

pub fn render_calculator_window(parent_height: i32, parent_width: i32) -> String {
    let window_width = 320;
    let window_height = 480;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    CalculatorWindow {
        top,
        left,
        window_width,
        window_height,
    }
    .render_once()
    .unwrap()
}
