use hypertext::{Raw, prelude::*};

pub fn render_flappy_bird_file() -> impl Renderable {
    maud! {
        div data-file-type="flappybird" class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 group-hover/item:bg-blue-900 p-1.5 rounded-xs w-24 h-fit file" {
            img class="w-9 h-9 select-none" src="/assets/images/flappy_bird.png" draggable="false";
            div class="text-white text-sm text-center select-none" {
                "Flappy Bird"
            }
        }
    }
}

pub fn render_flappy_bird_window(parent_height: i32, parent_width: i32) -> impl Renderable {
    let left = ((parent_width / 2) - (800 / 2)).max(0);
    let top = ((parent_height / 2) - (512 / 2)).max(0);

    maud! {
        (Raw::dangerously_create(r#"
            <script type="module">
                import {setupFlappyToolBar, setupFlappyWindowGrab} from "/assets/js/flappy.js";
                setupFlappyToolBar();
                setupFlappyWindowGrab();
            </script>
        "#))
        div
            id="flappy-canvas-container"
            class="absolute"
            style={ "top:" (top) "px; left:" (left) "px; width: 800px; height: 512px;" }
        {
            div id="flappy-toolbar" class="flex justify-between items-center bg-zinc-950 px-3 h-12 select-none" {
                div class="flex items-center gap-2" {
                    img class="w-5 h-5" src="/assets/images/flappy_bird.png" draggable="false";
                    div class="text-white" {
                        "Flappy Bird"
                    }
                }
                div class="flex items-center gap-3" {
                    img class="hover:opacity-70 w-5 h-5 hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 close" src="/assets/images/x.svg" draggable="false";
                }
            }
            canvas id="canvas" {}
        }
        (Raw::dangerously_create(r#"
            <script type="text/javascript">
                window.loadFlappyBirdModule();
            </script>
        "#))
    }
}
