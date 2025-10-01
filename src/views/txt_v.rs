use hypertext::{Raw, prelude::*};

pub fn render_txt_file(
    file_id: i32,
    file_name: &Option<String>,
    absolute: bool,
) -> impl Renderable {
    let file_name = file_name.as_deref().unwrap_or("New Text");

    let class = if absolute {
        "absolute inset-0 flex justify-center py-2"
    } else {
        "flex justify-center"
    };

    maud! {
        div
            id={ "file-" (file_id) }
            data-file-type="txt"
            class=(class)
        {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit min-w-[70px] h-fit cursor-pointer" {
                img class="w-9 h-9 select-none" src="/assets/images/text-editor.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis text-white text-sm text-center line-clamp-2 select-none" {
                    (file_name)
                }
            }
        }
    }
}

pub fn render_txt_input(file_id: i32, value: &str) -> impl Renderable {
    maud! {
        (Raw::dangerously_create(format!(
            r#"
                <script type="module">
                    const txtInput = document.getElementById("file-{file_id}").querySelector("textarea");
                    txtInput.focus();
                    txtInput.setSelectionRange(txtInput.value.length, txtInput.value.length);
                </script>
            "#
        )))
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
    let window_width = 600;
    let window_height = 500;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        (Raw::dangerously_create(format!(
            r#"
                <script type="module">
                    import {{setupTxtWindowGrab, setupTxtEditor, setupTxtToolBar}} from "/assets/js/txt.js";
                    setupTxtWindowGrab({txt_id});
                    setupTxtEditor({txt_id});
                    const txtEditor = document.getElementById("txt-editor-{txt_id}");
                    txtEditor.focus();
                    const range = document.createRange();
                    range.selectNodeContents(txtEditor);
                    range.collapse(false);
                    const selection = window.getSelection();
                    selection.removeAllRanges();
                    selection.addRange(range);
                    setupTxtToolBar({})
                </script>
            "#,
            txt_id
        )))
        div
            id={ "txt-window-" (txt_id) }
            class="absolute flex flex-col bg-zinc-950 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={ "txt-header-" (txt_id) } class="flex justify-between items-center bg-zinc-900 px-3 h-12 select-none" {
                div class="flex items-center gap-2" {
                    img class="w-5 h-5" src="/assets/images/text-editor.svg" draggable="false";
                    (file_name)
                }
                div class="flex items-center gap-3" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer maximize" src="/assets/images/square.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            div id={"txt-buttons-" (txt_id)} class="flex gap-3 bg-zinc-900 px-3 py-2 border-zinc-700 border-t border-b" {
                img class="hover:opacity-70 w-5 h-5 cursor-pointer bold" src="/assets/images/bold.svg" draggable="false";
                img class="hover:opacity-70 w-5 h-5 italic cursor-pointer" src="/assets/images/italic.svg" draggable="false";
                img class="hover:opacity-70 w-5 h-5 underline cursor-pointer" src="/assets/images/underline.svg" draggable="false";
            }
            div class="px-3 py-2 h-full" {
                div
                    id={"txt-editor-" (txt_id)}
                    class="focus-visible:border-none focus-visible:outline-none w-full h-full"
                    contenteditable="true"
                {
                    "Hello, world!"
                }
            }
        }
    }
}
