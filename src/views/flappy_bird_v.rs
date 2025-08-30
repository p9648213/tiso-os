use hypertext::{Raw, prelude::*};

pub fn render_flappy_bird_file() -> impl Renderable {
    maud! {
        div data-file-type="flappybird" class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-18 h-fit cursor-pointer file" {
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
        div id="flappy-canvas-container" class="absolute" style={ "top:" (top) "px; left:" (left) "px;" } {
            canvas id="canvas" {}
        }
        (Raw::dangerously_create(r#"
            <script type="text/javascript">
                window.loadFlappyBirdModule();
            </script>
        "#))
    }
}
