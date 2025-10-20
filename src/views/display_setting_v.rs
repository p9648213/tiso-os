use crate::{constant::EXAMPLE_COLORS, models::display_setting_db::BackgroundType};
use askama::Template;

#[derive(Template)]
#[template(path = "display_setting/display_setting_window.html")]
pub struct DisplaySettingWindow<'a> {
    pub top: i32,
    pub left: i32,
    pub width: i32,
    pub height: i32,
    pub background_type: BackgroundType,
    pub background_color: Option<&'a str>,
    pub example_colors: &'a [&'a str],
}

pub fn render_display_setting_window(
    parent_height: i32,
    parent_width: i32,
    background_type: BackgroundType,
    background_color: Option<String>,
) -> String {
    let width = 800;
    let height = 700;

    let left = ((parent_width / 2) - (width / 2)).max(0);
    let top = ((parent_height / 2) - (height / 2)).max(0);

    DisplaySettingWindow {
        top,
        left,
        width,
        height,
        background_type,
        background_color: background_color.as_deref(),
        example_colors: &EXAMPLE_COLORS,
    }
    .render()
    .unwrap()
}
