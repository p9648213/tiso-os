use crate::models::web_builder_db::DomTree;
use hypertext::{Raw, prelude::*};

pub fn render_web_builder_file(file_id: i32) -> impl Renderable {
    maud! {
        div
            id={"file-" {file_id}}
            data-file-type="web_builder"
            class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 group-hover/item:bg-blue-900 p-1.5 rounded-xs w-24 h-fit file"
        {
            img class="w-9 h-9 select-none" src="/assets/images/web-builder.svg" draggable="false";
            div class="max-w-[80px] overflow-ellipsis text-white text-sm text-center line-clamp-2 select-none" {
                "Web Builder"
            }
        }
    }
}

pub fn render_web_builder_window(
    web_builder_id: i32,
    file_name: &str,
    builder_name: &str,
    data: &DomTree,
    parent_height: i32,
    parent_width: i32,
) -> impl Renderable {
    let window_width = parent_width * 85 / 100;
    let window_height = parent_height * 96 / 100;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        (Raw::dangerously_create(format!(
            r#"
                <script type="module">
                    import {{setupWebBuilderToolBar}} from "/assets/js/web_builder.js";
                    setupWebBuilderToolBar({web_builder_id});
                </script>
            "#
        )))

        div
            id={ "web-builder-window-" (web_builder_id) }
            class="absolute flex flex-col rounded-sm overflow-hidden"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={ "web-builder-header-" (web_builder_id) } class="flex justify-between items-center bg-zinc-900 px-3 h-12 select-none" {
                div class="flex items-center gap-2 text-white" {
                    img class="w-5 h-5" src="/assets/images/web-builder.svg" draggable="false";
                    (file_name)
                }
                div class="flex items-center gap-3" {
                    img class="hover:opacity-70 w-5 h-5 hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 maximize" src="/assets/images/square.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 close" src="/assets/images/x.svg" draggable="false";
                }
            }

            div class="flex flex-col bg-zinc-100 h-full" {
                div class="px-3 py-1.5 border-zinc-950 border-b" {
                    "Page: " (builder_name)
                }

                div class="flex flex-1" {
                    (render_structure())
                    (render_review())
                    (render_setting())
                }
            }
        }
    }
}

fn render_structure() -> impl Renderable {
    maud! {
        div class="flex flex-col border-zinc-950 border-r w-70" {
            div class="flex-1 px-3 py-2 border-zinc-950 border-b" {
                "Web Tree"
            }
            div class="flex-1 px-3 py-2" {
                "Add Section"
            }
        }
    }
}

fn render_review() -> impl Renderable {
    maud! {
        div class="flex-1 px-3 py-2" { "Review" }
    }
}

fn render_setting() -> impl Renderable {
    maud! {
        div class="px-3 py-2 border-zinc-950 border-l w-70" { "Setting" }
    }
}
