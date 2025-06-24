use std::collections::HashMap;

use hypertext::{
    GlobalAttributes, HtmxAttributes, Raw, Renderable, html_elements, maud, maud_move,
};

use crate::{
    contanst::MIN_RECTANGLE_WIDTH,
    controllers::account_c::AccountForm,
    models::{
        desktop::{DesktopItem, ItemType},
        folders_db::FolderSortType,
    },
    utilities::screen_utils::parse_position,
    views::{folder_v::render_new_folder, txt_v::render_new_txt},
};

pub fn render_welcome_screen() -> impl Renderable {
    let account_form = AccountForm {
        username: "".to_string(),
        password: "".to_string(),
        confirm_password: None,
    };

    maud_move!(
        (Raw(r#"<!DOCTYPE html>"#))
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                link rel="stylesheet" href="/assets/css/lib/tailwind.css";
                link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico";
                script src="/assets/js/lib/htmx.js" defer {}
                script src="/assets/js/main.js" type="module" defer {}
            }
            title { "TisoOS - Welcome" }
            body style="background: radial-gradient(ellipse at top left, #070f2b, #1b1a55, #535c91);" class="relative overflow-hidden" {
                main class="relative flex flex-col justify-center items-center min-h-screen text-white text-center" {
                    div class="mb-8" {
                        div class="flex justify-center items-center bg-white bg-opacity-20 backdrop-blur-sm mx-auto mb-6 rounded-2xl w-24 h-24" {
                            div class="flex justify-center items-center bg-white rounded-lg w-12 h-12" {
                                div class="bg-blue-600 rounded-sm w-6 h-6" {}
                            }
                        }
                        h1 class="mb-2 font-light text-4xl" { "TisoOS" }
                        p class="font-light text-lg" { "Your gateway to productivity" }
                    }
                    div class="mb-8 max-w-md" {
                        h2 class="mb-4 font-light text-3xl" { "Welcome" }
                        p class="text-lg leading-relaxed" {
                            "Let's get you set up and ready to go. This will only take a few minutes."
                        }
                    }
                    (render_account_form(&account_form, true))
                }
            }
        }
    )
}

pub fn render_account_form(account_form: &AccountForm, register_mode: bool) -> impl Renderable {
    maud_move! {
        form
            id="account_form"
            hx-post="/create/account"
            hx-trigger="submit"
            hx-target="body"
            class="relative flex flex-col space-y-2 text-black"
        {

            input
                class="bg-white px-3 rounded-sm h-8"
                id="account_username"
                name="username"
                placeholder="Username"
                value=(account_form.username)
                autofocus="true"
                autocomplete="off"
                hx-post="/create/account"
                hx-swap=(if register_mode { "outerHTML" } else { "none" })
                hx-target=(if register_mode { "#account_confirm_password" } else { "" })
                hx-trigger="input delay:300ms";
            input
                class="bg-white px-3 rounded-sm h-8"
                name="password" placeholder="Password"
                type="password"
                value=(account_form.password);
            (render_comfirm_password(&account_form.confirm_password, register_mode))
            div id="account_error" class="-bottom-6.5 absolute text-red-400" {}
            button type="submit" class="hidden" {}
        }
    }
}

pub fn render_comfirm_password(value: &Option<String>, register_mode: bool) -> impl Renderable {
    maud_move! {
        input
            class="bg-white px-3 rounded-sm h-8"
            name="confirm_password"
            id="account_confirm_password"
            placeholder="Confirm Password"
            type=(if register_mode { "password" } else { "hidden" })
            value=(value)
            hx-post="/create/account"
            hx-target=(if register_mode { "#account_error" } else { "" })
            hx-trigger="input delay:300ms";
    }
}

pub fn render_screen() -> impl Renderable {
    maud! {
        (Raw(r#"<!DOCTYPE html>"#))
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                link rel="stylesheet" href="/assets/css/lib/tailwind.css";
                link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico";
                script src="/assets/js/lib/htmx.js" defer {}
                script src="/assets/js/main.js" type="module" defer {}
            }
            title { "TisoOS" }
            body style="background: radial-gradient(ellipse at top left, #070f2b, #1b1a55, #535c91);" class="relative overflow-hidden" {
                (render_screen_section())
            }
        }
    }
}

pub fn render_screen_section() -> impl Renderable {
    maud! {
        (Raw(r#"
            <script type="module">
                import {setupDesktopContextMenu} from "/assets/js/context_menu.js";
                import {setupGridDimensions} from "/assets/js/grid.js";
                import {setupResize} from "/assets/js/resize.js";
                import {setupDesktopDrag} from "/assets/js/drag.js";
                setupDesktopContextMenu();
                setupGridDimensions();
                setupResize();
                setupDesktopDrag();
            </script>
        "#))
        main class="flex flex-wrap h-[calc(100%-theme('spacing.12'))]" {}
        footer class="right-0 bottom-0 left-0 absolute bg-zinc-800 border-t border-t-zinc-700 h-12" {}
    }
}

pub fn render_screen_grid(
    height: u16,
    width: u16,
    desktop_id: i32,
    sort_type: &FolderSortType,
    items: Vec<DesktopItem>,
) -> impl Renderable {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1;

    maud_move! {
        input id="screen_rows" type="hidden" value=(rows);
        input id="screen_cols" type="hidden" value=(cols);
        input id="desktop_id" type="hidden" value=(desktop_id);

        @match *sort_type {
            FolderSortType::Custom => {
                @let item_map: HashMap<(u16, u16), &DesktopItem> = items
                    .iter()
                    .filter_map(|item| {
                        item.desktop_position
                            .as_deref()
                            .and_then(parse_position)
                            .map(|pos| (pos, item))
                    })
                    .collect();

                @for row in 0..rows {
                    @for col in 0..cols {
                        div
                            class = "flex justify-center items-center relative"
                            style={ "width:" (rectangle_width) "px;" }
                            id={ "item-" (row) "-" (col) }
                            draggable="true"
                        {
                            @if let Some(item) = item_map.get(&(row, col)) {
                                @match item.item_type.as_ref().expect("No item_type column or value is null") {
                                    ItemType::File => {
                                        (render_new_txt(item.id.expect("No id column or value is null")))
                                    }
                                    ItemType::Folder => {
                                        (render_new_folder(item.id.expect("No id column or value is null")))
                                    }
                                }
                            }
                        }
                    }
                }
            },
            FolderSortType::DateCreated => {
                @for row in 0..rows {
                    @for col in 0..cols {
                        div
                            class = "flex justify-center items-center relative"
                            style={ "width:" (rectangle_width) "px;" }
                            id={ "item-" (row) "-" (col) }
                            draggable="true"
                        {
                            @if let Some(item) = items.get((col * rows + row) as usize) {
                                @match item.item_type.as_ref().expect("No item_type column or value is null") {
                                    ItemType::File => {
                                        (render_new_txt(item.id.expect("No id column or value is null")))
                                    }
                                    ItemType::Folder => {
                                        (render_new_folder(item.id.expect("No id column or value is null")))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
