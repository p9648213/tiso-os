use hypertext::{GlobalAttributes, HtmxAttributes, Raw, Renderable, html_elements, maud_move};

pub fn render_folder(folder_id: i32, folder_name: &Option<String>) -> impl Renderable {
    let folder_name = folder_name.as_deref().unwrap_or("New Folder");

    maud_move! {
        (Raw(format!(r#"
            <script>
                document.getElementById("folder-{}").parentNode.draggable = true;
            </script>
        "#, folder_id)))
        div id={ "folder-" (folder_id) } class="absolute inset-0 flex justify-center py-2" {
            div class="flex flex-col justify-center items-center h-fit gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/folder.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis overflow-hidden text-white text-sm text-center select-none" {
                    (folder_name)
                }
            }
        }
    }
}

pub fn render_folder_input(folder_id: i32, value: &str) -> impl Renderable {
    maud_move! {
        (Raw(format!(r#"
            <script>
                document.getElementById("folder-{}").parentNode.draggable = false;
            </script>
        "#, folder_id)))
        div id={ "folder-" (folder_id) } class="absolute inset-0 flex justify-center" {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/folder.svg" draggable="false";
                input
                    hx-post={"/update/folder/rename/" (folder_id)}
                    hx-target={"#folder-" (folder_id)}
                    hx-swap="outerHTML"
                    hx-trigger="blur"
                    name="folder_name"
                    type="text"
                    class="max-w-[75px] text-white text-sm text-center"
                    value=(value)
                    autofocus="true";
            }
        }
    }
}

