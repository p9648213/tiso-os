use hypertext::{Raw, prelude::*};

use crate::{constant::EXAMPLE_COLORS, models::display_setting_db::BackgroundType};

pub fn render_display_setting_window(
    parent_height: i32,
    parent_width: i32,
    background_type: BackgroundType,
    background_color: Option<String>,
) -> impl Renderable {
    let window_width = 800;
    let window_height = 700;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        (Raw::dangerously_create(r#"
            <script type="module">
                import {setupSelectBackgroundType, setupBackgroundColorList, setupSelectBackgroundPicture} from "/assets/js/display_setting.js";
                setupSelectBackgroundType();
                setupBackgroundColorList();
                setupSelectBackgroundPicture();
            </script>
        "#))
        div
            id={"display-setting"}
            class="absolute flex flex-col bg-zinc-950 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={"display-setting-header"} class="flex justify-between items-center bg-zinc-900 px-3 h-12 select-none" {
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
                        class="bg-zinc-700 px-3 py-1 rounded-sm h-7"
                    {
                        @if background_type == BackgroundType::SolidColor {
                            option selected value="SolidColor" { "Solid Color" }
                        } @else {
                            option value="SolidColor" { "Solid Color" }
                        }

                        @if background_type == BackgroundType::Picture {
                            option selected value="Picture" { "Picture" }
                        } @else {
                            option value="Picture" { "Picture" }
                        }
                    }
                }
                div id="display-setting-background-color" class=(if background_type == BackgroundType::SolidColor {"flex justify-between items-center bg-zinc-800 p-2 rounded-sm" } else { "hidden justify-between items-center bg-zinc-800 p-2 rounded-sm" }) {
                    div {
                        "Choose your background color"
                    }
                    div id="background-color-list" class="flex gap-2" {
                        @if let Some(background_color) = &background_color {
                            @for color in EXAMPLE_COLORS {
                                div
                                    data-color=(color)
                                    class="rounded-sm w-6 h-6 cursor-pointer"
                                    style={
                                        "background:" (color) ";"
                                        "outline:" (if background_color == color { "3px solid #155dfc" } else { "none" })
                                    } {}
                            }
                        }
                    }
                }
                div id="display-setting-background-picture" class=(if background_type == BackgroundType::Picture {"flex justify-between items-center bg-zinc-800 p-2 rounded-sm" } else { "hidden justify-between items-center bg-zinc-800 p-2 rounded-sm" }) {
                    div {
                        "Choose your background picture"
                    }
                    form
                        hx-post="update/setting/display/background_picture"
                        hx-encoding="multipart/form-data"
                        hx-swap="outerHTML"
                        hx-target="#background-container"
                        hx-trigger="change"
                    {
                        input type="file" id="background-picture" name="background-picture" accept="image/*" required="true" class="hidden";
                        div class="flex items-center gap-2" {
                            button type="button" class="bg-zinc-700 rounded-sm h-7" {
                                label for="background-picture" class="px-3 cursor-pointer" { "Choose Image" }
                            }
                            div class="max-w-40 truncate" id="background-picture-name" { "No image selected" }
                        }
                    }
                }
            }
        }
    }
}
