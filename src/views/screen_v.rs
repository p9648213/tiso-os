use crate::{
    constant::MIN_RECTANGLE_WIDTH,
    models::{
        folder_db::FolderSortType,
        folder_item::FolderItem,
    },
    utilities::common::parse_position,
};
use askama::Template;
use std::collections::HashMap;

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

#[derive(Template)]
#[template(path = "screen/screen_grid.html")]
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
    .render()
    .unwrap()
}
