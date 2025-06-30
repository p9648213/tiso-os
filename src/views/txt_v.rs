use hypertext::{GlobalAttributes, HtmxAttributes, Raw, Renderable, html_elements, maud_move};

pub fn render_txt(file_id: i32, file_name: &Option<String>) -> impl Renderable {
    let file_name = file_name.as_deref().unwrap_or("New Text");

    maud_move! {
        (Raw(format!(r#"
            <script>
                document.getElementById("file-{}").parentNode.draggable = true;
            </script>
        "#, file_id)))
        div id={ "file-" (file_id) } data-file-type="txt" class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/text-editor.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis overflow-hidden text-white text-sm text-center select-none" {
                    (file_name)
                }
            }
        }
    }
}

pub fn render_txt_input(file_id: i32, value: &str) -> impl Renderable {
    maud_move! {
        (Raw(format!(r#"
            <script>
                document.getElementById("file-{}").parentNode.draggable = false;
            </script>
        "#, file_id)))
        div id={ "file-" (file_id) } data-file-type="txt" class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/text-editor.svg" draggable="false";
                input
                    hx-post={"/update/file/rename/txt/" (file_id)}
                    hx-target={"#file-" (file_id)}
                    hx-swap="outerHTML"
                    hx-trigger="blur"
                    name="file_name"
                    type="text"
                    class="max-w-[75px] text-white text-sm text-center"
                    value=(value)
                    autofocus="true";
            }
        }
    }
}
