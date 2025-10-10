use hypertext::prelude::*;

pub fn render_thispc_file(file_id: i32, file_name: &Option<String>) -> impl Renderable {
    let file_name = file_name.as_deref().unwrap_or("This PC");

    maud! {
        div
            id={ "folder-" (file_id) }
            data-folder-type="Root"
            class="absolute inset-0 flex justify-center py-2"
        {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 group-hover/item:bg-blue-900 p-1.5 rounded-xs w-24 min-w-[70px] h-fit" {
                img class="w-10 h-10 select-none" src="/assets/images/thispc.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis text-white text-sm text-center line-clamp-2 select-none" {
                    (file_name)
                }
            }
        }
    }
}
