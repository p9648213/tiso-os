use hypertext::{
    GlobalAttributes, HtmxAttributes, Raw, Renderable, html_elements, maud, maud_move,
};

use crate::contanst::MIN_RECTANGLE_WIDTH;

use super::ItemType;

pub fn render_screen() -> impl Renderable {
    maud! {
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                link rel="stylesheet" href="/assets/css/lib/tailwind.css";
                link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico";
                script src="/assets/js/lib/htmx.js" defer {}
                script src="/assets/js/main.js" type="module" defer {}
            }
            title { "TisoOS" }
            body class="relative bg-zinc-900 overflow-hidden" {
                (Raw(r#"
                    <script type="module">
                        import {setupRightClickContextMenu} from "/assets/js/right_click.js";
                        setupRightClickContextMenu();
                    </script>
                "#))
                main class="flex flex-wrap h-[calc(100%-theme('spacing.12'))]" {}
                footer class="right-0 bottom-0 left-0 absolute bg-zinc-800 border-t border-t-zinc-700 h-12" {}
                form
                    hx-trigger="load"
                    hx-post="/action/create-grid"
                    hx-target="main"
                {
                    input id="main_height" name="height" type="hidden";
                    input id="main_width" name="width" type="hidden";
                }
            }
        }
    }
}

pub fn render_screen_grid(height: u16, width: u16) -> impl Renderable {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1;

    maud_move! {
        input id="screen_rows" type="hidden" value=(rows);
        input id="screen_cols" type="hidden" value=(cols);

        @for row in 0..rows {
            @for col in 0..cols {
                div class = "flex justify-center items-center relative"
                    style={ "width:" (rectangle_width) "px;" }
                    id={ "item-" (row) "-" (col) } {}
            }
        }
    }
}

pub fn render_screen_item(item_type: ItemType) -> impl Renderable {
    maud_move! {
        div class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1 hover:bg-blue-900 p-1.5 rounded-xs w-fit" {
                @match item_type {
                    ItemType::Text => {
                        img class="w-9 h-9" src="/assets/images/text-editor.svg";
                        div class="max-w-20 overflow-ellipsis overflow-hidden text-white text-sm whitespace-nowrap select-none" {
                            "New Text.txt"
                        }
                    }
                    ItemType::Folder => {
                        img class="w-9 h-9" src="/assets/images/folder.svg";
                        div class="max-w-20 overflow-ellipsis overflow-hidden text-white text-sm whitespace-nowrap select-none" {
                            "New Folder"
                        }
                    }
                }
            }
        }
    }
}
