use hypertext::prelude::*;

pub fn render_display_setting_window(parent_height: i32, parent_width: i32) -> impl Renderable {
    let window_width = 800;
    let window_height = 700;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
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
            div class="px-3 py-2 h-full" {
                "Display Settings"
            }
        }
    }
}
