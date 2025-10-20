use askama::Template;

use crate::models::web_builder_db::DomTree;

#[derive(Template)]
#[template(path = "web_builder/web_builder_file.html")]
struct WebBuilderFile {
    pub id: i32,
}

pub fn render_web_builder_file(file_id: i32) -> String {
    WebBuilderFile { id: file_id }.render().unwrap()
}

#[derive(Template)]
#[template(path = "web_builder/web_builder_window.html")]
struct WebBuilderWindow<'a> {
    pub web_builder_id: i32,
    pub file_name: &'a str,
    pub builder_name: &'a str,
    pub data: &'a DomTree,
    pub window_width: i32,
    pub window_height: i32,
    pub top: i32,
    pub left: i32,
}

pub fn render_web_builder_window(
    web_builder_id: i32,
    file_name: &str,
    builder_name: &str,
    data: &DomTree,
    parent_height: i32,
    parent_width: i32,
) -> String {
    let window_width = parent_width * 85 / 100;
    let window_height = parent_height * 96 / 100;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    WebBuilderWindow {
        web_builder_id,
        file_name,
        builder_name,
        data,
        window_width,
        window_height,
        top,
        left,
    }
    .render()
    .unwrap()
}

#[derive(Template)]
#[template(path = "web_builder/web_builder_structure.html")]
struct WebBuilderStructure {}

pub fn render_web_builder_structure() -> String {
    WebBuilderStructure {}.render().unwrap()
}

#[derive(Template)]
#[template(path = "web_builder/web_builder_review.html")]
struct WebBuilderReview {}

pub fn render_web_builder_review() -> String {
    WebBuilderReview {}.render().unwrap()
}

#[derive(Template)]
#[template(path = "web_builder/web_builder_setting.html")]
struct WebBuilderSetting {}

pub fn render_web_builder_setting() -> String {
    WebBuilderSetting {}.render().unwrap()
}
