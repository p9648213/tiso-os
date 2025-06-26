use hypertext::{GlobalAttributes, Renderable, html_elements, maud_move};

pub fn render_txt(file_id: i32) -> impl Renderable {
    maud_move! {
        div id={ "file-" (file_id) } data-file-type="txt" class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/text-editor.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis overflow-hidden text-white text-sm text-center select-none" {
                    "New Text"
                }
            }
        }
    }
}

pub fn render_txt_input(file_id: i32, value: &str) -> impl Renderable {
    maud_move! {
        div id={ "file-" (file_id) } data-file-type="txt" class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/text-editor.svg" draggable="false";
                input type="text" class="max-w-[75px] text-white text-sm text-center" value=(value) autofocus="true";
            }
        }
    }
}
