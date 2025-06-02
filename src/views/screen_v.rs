use hypertext::{GlobalAttributes, Renderable, html_elements, maud_move, rsx_move};
use vy::prelude::*;

use crate::contanst::MIN_RECTANGLE_WIDTH;

use super::ItemType;

pub fn render_screen() -> impl IntoHtml {
    (
        DOCTYPE,
        html!(
            head!(
                meta!(charset = "UTF-8"),
                meta!(
                    name = "viewport",
                    content = "width=device-width,initial-scale=1"
                ),
                link!(rel = "stylesheet", href = "/assets/css/lib/tailwind.css"),
                link!(
                    rel = "icon",
                    "type" = "image/x-icon",
                    href = "/assets/images/favicon.ico"
                ),
                script!(src = "/assets/js/lib/htmx.js", defer = "defer"),
                script!(
                    src = "/assets/js/main.js",
                    "type" = "module",
                    defer = "defer"
                )
            ),
            title!("TisoOS"),
            body!(
                class = "bg-zinc-900 relative",
                PreEscaped(
                    r#"
                      <script type="module">
                          import {setupRightClickContextMenu} from "/assets/js/right_click.js";
                          setupRightClickContextMenu();
                      </script>
                    "#
                ),
                main!(class = "flex flex-wrap h-[calc(100%-theme('spacing.12'))]"),
                footer!(
                    class = "right-0 bottom-0 left-0 absolute border-t-zinc-700 bg-zinc-800 border-t h-12"
                ),
                form!(
                    "hx-trigger" = "load",
                    "hx-post" = "/action/create-grid",
                    "hx-target" = "main",
                    input!(id = "main_height", name = "height", "type" = "hidden"),
                    input!(id = "main_width", name = "width", "type" = "hidden"),
                )
            )
        ),
    )
}

pub fn render_screen_grid(height: u16, width: u16) -> impl IntoHtml {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1;

    (
        input!(id = "screen_rows", "type" = "hidden", value = rows),
        input!(id = "screen_cols", "type" = "hidden", value = cols),
        (0..rows).map(move |row| {
            (0..cols).map(move |col| {
                div!(
                    class = "flex justify-center items-center relative",
                    style = format!("width:{}px;", rectangle_width),
                    id = format!("item-{}-{}", row, col)
                )
            })
        }),
    )
}

pub fn render_screen_grid_2(height: u16, width: u16) -> maud::Markup {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1;

    maud::html! {
        input id="screen_rows" type="hidden" value=(rows);
        input id="screen_cols" type="hidden" value=(cols);

        @for row in 0..rows {
            @for col in 0..cols {
                div class="flex justify-center items-center relative"
                     style={ "width:" (rectangle_width) "px;" }
                     id={ "item-" (row) "-" (col) } {}
            }
        }
    }
}

pub fn render_screen_grid_3(height: u16, width: u16) -> impl Renderable {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1;

    maud_move! {
        input id="screen_rows" type="hidden" value=(rows);
        input id="screen_cols" type="hidden" value=(cols);

        @for row in 0..rows {
            @for col in 0..cols {
                div class = "flex justify-center items-center relative"
                    style = (format!("width: {}px", rectangle_width))
                    id = (format!("item-{}-{}", row, col)) {}
            }
        }
    }
}

pub fn render_screen_grid_4(height: u16, width: u16) -> impl Renderable {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1; 

    rsx_move! {
        <input id="screen_rows" type="hidden" value={rows}>
        <input id="screen_cols" type="hidden" value={cols}>
        @for row in 0..rows {
            @for col in 0..cols {
                <div
                    class="flex justify-center items-center relative"
                    style={format!("width: {}px", rectangle_width)}
                    id={format!("item-{}-{}", row, col)}>
                </div>
            }
        }
    }
}

pub fn render_screen_item(item_type: ItemType) -> impl IntoHtml {
    match item_type {
        ItemType::Text => div!(
            class = "absolute inset-0 flex items-center justify-center",
            div!(
                class = "flex flex-col items-center justify-center gap-1 w-fit p-1.5 hover:bg-blue-900 rounded-xs",
                img!(class = "w-9 h-9", src = "/assets/images/text-editor.svg"),
                div!(
                    class = "whitespace-nowrap overflow-hidden overflow-ellipsis max-w-20 text-white text-sm select-none",
                    "New Text.txt"
                )
            )
        ),
        ItemType::Folder => div!(
            class = "absolute inset-0 flex items-center justify-center",
            div!(
                class = "flex flex-col items-center justify-center gap-1 w-fit p-1.5 hover:bg-blue-900 rounded-xs",
                img!(class = "w-9 h-9", src = "/assets/images/folder.svg"),
                div!(
                    class = "whitespace-nowrap overflow-hidden overflow-ellipsis max-w-20 text-white text-sm select-none",
                    "New Folder"
                )
            )
        ),
    }
}
