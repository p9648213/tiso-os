use hypertext::{GlobalAttributes, Raw, Renderable, html_elements, maud_move};

pub fn render_folder(folder_id: i32) -> impl Renderable {
    maud_move! {
        (Raw(format!(r#"
            <script>
                document.getElementById("folder-{}").parentNode.draggable = false;
            </script>
        "#, folder_id)))
        div id={ "folder-" (folder_id) } class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-fit cursor-pointer" {
                img class="w-9 h-9" src="/assets/images/folder.svg" draggable="false";
                div class="max-w-[75px] overflow-ellipsis overflow-hidden text-white text-sm text-center select-none" {
                    "New Folder"
                }
            }
        }
    }
}
