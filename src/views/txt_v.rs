use hypertext::{GlobalAttributes, HtmxAttributes, Raw, Renderable, html_elements, maud_move};

pub fn render_txt_file(file_id: i32, file_name: &Option<String>) -> impl Renderable {
    let file_name = file_name.as_deref().unwrap_or("New Text");

    maud_move! {
        div
            id={ "file-" (file_id) }
            data-file-type="txt"
            class="absolute inset-0 flex justify-center py-2"
        {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit h-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/text-editor.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis text-white text-sm text-center line-clamp-2 select-none" {
                    (file_name)
                }
            }
        }
    }
}

pub fn render_txt_input(file_id: i32, value: &str) -> impl Renderable {
    maud_move! {
        div
            id={ "file-" (file_id) }
            data-file-type="txt"
            class="absolute inset-0 flex justify-center py-2"
        {
            div class="flex flex-col justify-center items-center gap-1.5 p-1.5 rounded-xs w-fit h-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/text-editor.svg" draggable="false";
                textarea
                    hx-post={"/update/file/rename/txt/" (file_id)}
                    hx-target={"#file-" (file_id)}
                    hx-swap="outerHTML"
                    hx-trigger="blur, keyup[key=='Enter']"
                    hx-on:blur="window.editMode = false"
                    hx-on:keydown=r#"if(event.key == 'Enter') { event.preventDefault(); window.editMode = false }"#
                    name="file_name"
                    class="max-w-[75px] overflow-hidden text-white text-sm text-center resize-none"
                    autofocus="true"
                {
                    (value)
                }
            }
        }
    }
}

pub fn render_txt_window(
    file_name: &str,
    txt_id: i32,
    parent_height: i32,
    parent_width: i32,
) -> impl Renderable {
    let window_width = 300;
    let window_height = 200;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud_move! {
        (Raw(format!(
            r#"
                <script type="module">
                    import {{setupTxtWindowGrab}} from "/assets/js/txt.js";
                    setupTxtWindowGrab({});
                </script>
            "#, txt_id
        )))
        div
            id={ "txt-window-" (txt_id) }
            class="absolute bg-white shadow-lg"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={ "txt-header-" (txt_id) } class="flex items-center bg-red-300 px-2 h-5 select-none" {
                (file_name)
            }
            div class="p-2" { "Content" }
        }
    }
}
