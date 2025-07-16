use hypertext::{GlobalAttributes, Raw, Renderable, html_elements, maud_move};

pub fn render_snake_file() -> impl Renderable {
    maud_move! {
        div data-file-type="snake" class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-18 h-fit cursor-pointer" {
            img class="w-9 h-9 select-none" src="/assets/images/snake.svg" draggable="false";
            div class="text-white text-sm text-center select-none" {
                "Snake"
            }
        }
    }
}

pub fn render_snake_window() -> impl Renderable {
    maud_move! {
        div id="canvas-container" class="top-1/2 left-1/2 absolute -translate-x-1/2 -translate-y-1/2" {
            canvas id="canvas" {}
        }
        (Raw(r#"
            <script type="text/javascript">
                window.loadSnakeModule();
            </script>
        "#))
    }
}
