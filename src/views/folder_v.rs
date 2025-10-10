use hypertext::prelude::*;

pub fn render_folder(
    folder_id: i32,
    folder_name: &Option<String>,
    id_prefix: &Option<String>,
) -> impl Renderable {
    let folder_name = folder_name.as_deref().unwrap_or("New Folder");

    let id = match id_prefix {
        Some(prefix) => format!("{prefix}-folder-{folder_id}"),
        None => format!("folder-{folder_id}"),
    };

    maud! {
        div
            id=(id)
            data-folder-type="Normal"
            class="absolute inset-0 flex justify-center py-2"
        {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 group-hover/item:bg-blue-900 p-1.5 rounded-xs w-24 min-w-[70px] h-fit" {
                img class="w-9 h-9 select-none" src="/assets/images/folder.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis text-white text-sm text-center line-clamp-2 select-none" {
                    (folder_name)
                }
            }
        }
    }
}

pub fn render_folder_input(folder_id: i32, value: &str) -> impl Renderable {
    maud! {
        div
            id={ "folder-" (folder_id) }
            class="absolute inset-0 flex justify-center py-2"
        {
            div class="flex flex-col justify-center items-center gap-1.5 p-1.5 rounded-xs w-fit h-fit" {
                img class="w-9 h-9" src="/assets/images/folder.svg" draggable="false";
                textarea
                    hx-post={"/update/folder/rename/" (folder_id)}
                    hx-target={"#folder-" (folder_id)}
                    hx-swap="outerHTML"
                    hx-trigger="blur, keyup[key=='Enter']"
                    hx-on:blur="window.editMode = false"
                    hx-on:keydown=r#"if(event.key == 'Enter') { event.preventDefault(); window.editMode = false }"#
                    name="folder_name"
                    class="max-w-[75px] overflow-hidden text-white text-sm text-center resize-none"
                    autofocus="true"
                {
                    (value)
                }
            }
        }
    }
}
