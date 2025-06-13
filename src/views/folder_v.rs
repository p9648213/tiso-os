use hypertext::{GlobalAttributes, Renderable, html_elements, maud_move};

pub fn render_new_folder(file_id: i32) -> impl Renderable {
    maud_move! {
        div id=(file_id) class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1 hover:bg-blue-900 p-1.5 rounded-xs w-fit" {
                img class="w-9 h-9" src="/assets/images/folder.svg";
                div class="max-w-[75px] overflow-ellipsis overflow-hidden text-white text-sm text-center select-none" {
                    "New Folder"
                }
            }
        }
    }
}
