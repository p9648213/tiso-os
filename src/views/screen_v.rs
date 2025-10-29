use std::collections::HashMap;

use sailfish::TemplateSimple;

use crate::{
    constant::MIN_RECTANGLE_WIDTH,
    models::{
        folder_db::FolderSortType,
        folder_item::{FolderItem, ItemType},
        file_db::FileType,
    },
    utilities::common::parse_position,
};

#[derive(TemplateSimple)]
#[template(path = "welcome_screen.stpl")]
struct WelcomeScreen {}

pub fn render_welcome_screen() -> String {
    WelcomeScreen {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "confirm_password.stpl")]
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

#[derive(TemplateSimple)]
#[template(path = "main_screen.stpl")]
struct MainScreen<'a> {
    background: &'a str,
}

pub fn render_main_screen(background: &str) -> String {
    MainScreen { background }.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "screen_background.stpl")]
struct ScreenBackground<'a> {
    background: &'a str,
}

pub fn render_screen_background(background: &str) -> String {
    ScreenBackground { background }.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "screen_section.stpl")]
struct ScreenSection {}

pub fn render_screen_section() -> String {
    ScreenSection {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "screen_grid.stpl")]
struct ScreenGrid<'a> {
    desktop_id: i32,
    sort_type: &'a FolderSortType,
    items: &'a Vec<FolderItem>,
    item_map: HashMap<(u16, u16), &'a FolderItem>,
    rows: u16,
    cols: u16,
    rectangle_width: f32,
}

pub fn render_screen_grid(
    height: u16,
    width: u16,
    desktop_id: i32,
    sort_type: &FolderSortType,
    items: Vec<FolderItem>,
) -> String {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1;

    let item_map: HashMap<(u16, u16), &FolderItem> = items
        .iter()
        .filter_map(|item| {
            item.desktop_position
                .as_deref()
                .and_then(parse_position)
                .map(|pos| (pos, item))
        })
        .collect();

    ScreenGrid {
        desktop_id,
        sort_type,
        items: &items,
        item_map,
        rows,
        cols,
        rectangle_width,
    }
    .render_once()
    .unwrap()
}
