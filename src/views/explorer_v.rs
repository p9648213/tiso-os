use hypertext::prelude::*;

pub fn render_explorer(
    folder_id: i32,
    folder_name: String,
    parent_width: i32,
    parent_height: i32,
) -> impl Renderable {
    let window_width = 1320;
    let window_height = 800;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        div
            id={ "explorer-window-" (folder_id) }
            class="absolute flex flex-col bg-zinc-950 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={ "explorer-toolbar-" (folder_id) } class="flex justify-between bg-zinc-900 px-3 h-12 select-none" {
                div class="flex items-center gap-2 bg-zinc-800 mt-3 px-4 rounded-t-sm" {
                    img class="w-5 h-5" src="/assets/images/folder.svg" draggable="false";
                    (folder_name)
                }
                div class="flex items-center gap-3 h-full" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer maximize" src="/assets/images/square.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            div class="flex justify-between items-center bg-zinc-800 px-3 h-12 select-none" {

            }
            div class="px-3 py-2 h-full" {
                "Hi"
            }
        }
    }
}
