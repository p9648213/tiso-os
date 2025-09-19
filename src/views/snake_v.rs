use hypertext::{Raw, prelude::*};

pub fn render_snake_file() -> impl Renderable {
    maud! {
        div data-file-type="snake" class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-18 h-fit cursor-pointer file" {
            img class="w-9 h-9 select-none" src="/assets/images/snake.svg" draggable="false";
            div class="text-white text-sm text-center select-none" {
                "Snake"
            }
        }
    }
}

pub fn render_snake_window(parent_height: i32, parent_width: i32) -> impl Renderable {
    let left = ((parent_width / 2) - (900 / 2)).max(0);
    let top = ((parent_height / 2) - (600 / 2)).max(0);

    maud! {
        (Raw::dangerously_create(r#"
            <script type="module">
                import {setupSnakeToolBar} from "/assets/js/snake.js";
                setupSnakeToolBar();
            </script>
        "#))
        div id="snake-canvas-container" class="absolute" style={ "top:" (top) "px; left:" (left) "px;" } {
            div class="flex justify-between items-center bg-zinc-950 px-3 h-12 select-none" {
                div class="flex items-center gap-2" {
                    img class="w-5 h-5" src="/assets/images/snake.svg" draggable="false";
                    div class="text-white" {
                        "Snake"
                    }
                }
                div id="snake-toolbar" class="flex items-center gap-3" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            canvas id="canvas" {}
        }
        (Raw::dangerously_create(r#"
            <script type="text/javascript">
                window.loadSnakeModule();
            </script>
        "#))
    }
}
