use hypertext::{prelude::*, Raw};

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
        div id="snake-canvas-container" class="absolute" style={ "top:" (top) "px; left:" (left) "px;" } {
            canvas id="canvas" {}
        }
        (Raw::dangerously_create(r#"
            <script type="text/javascript">
                window.loadSnakeModule();
            </script>
        "#))
    }
}
