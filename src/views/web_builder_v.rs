use hypertext::prelude::*;

pub fn render_web_builder_file(file_id: i32) -> impl Renderable {
    maud! {
        div
            id={"file-" {file_id}}
            data-file-type="WebBuilder"
            class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 group-hover/item:bg-blue-900 p-1.5 rounded-xs w-24 h-fit file"
        {

            img class="w-9 h-9 select-none" src="/assets/images/web-builder.svg" draggable="false";
            div class="max-w-[80px] overflow-ellipsis text-white text-sm text-center line-clamp-2 select-none" {
                "Web Builder"
            }

        }
    }
}
