use hypertext::{Raw, prelude::*};

use crate::models::display_setting_db::BackgroundType;

pub fn render_display_setting_window(
    parent_height: i32,
    parent_width: i32,
    background_type: BackgroundType,
    background_picture: Option<Vec<u8>>,
    background_color: Option<String>,
) -> impl Renderable {
    let window_width = 800;
    let window_height = 700;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        (Raw::dangerously_create(r#"
            <script type="module">
                import {setupSelectBackgroundType} from "/assets/js/display_setting.js";
                setupSelectBackgroundType();
            </script>
        "#))
        div
            id={"display-setting"}
            class="absolute flex flex-col bg-zinc-900 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={"display-setting-header"} class="flex justify-between items-center bg-zinc-950 px-3 h-12 select-none" {
                div class="flex items-center" {
                    "Display Setting"
                }
                div class="flex items-center gap-3" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer maximize" src="/assets/images/square.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            div class="flex flex-col gap-2 px-3 py-2 h-full" {
                div class="flex justify-between bg-zinc-800 p-2 rounded-sm" {
                    div {
                        "Persionalize your background"
                    }
                    select
                        id="display-setting-background-type"
                        class="bg-zinc-700 px-3 py-1 rounded-sm"
                    {
                        option selected=(background_type == BackgroundType::SolidColor) value="SolidColor" { "Solid Color" }
                        option selected=(background_type == BackgroundType::Picture) value="Picture" { "Picture" }
                    }
                }
                div id="display-setting-background-color" class="flex bg-zinc-800 p-2 rounded-sm" {
                    div {
                        "Choose your background color"
                    }
                }
                div id="display-setting-background-picture" class="hidden bg-zinc-800 p-2 rounded-sm" {
                    div {
                        "Choose your background picture"
                    }
                }
            }
        }
    }
}
