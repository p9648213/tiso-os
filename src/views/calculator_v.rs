use hypertext::{GlobalAttributes, Renderable, html_elements, maud_move};

pub fn render_calculator_file() -> impl Renderable {
    maud_move! {
        div data-file-type="calculator" class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-18 h-fit cursor-pointer" {
            img class="w-9 h-9 select-none" src="/assets/images/calculator.svg" draggable="false";
            div class="text-white text-sm text-center select-none" {
                "Calculator"
            }
        }
    }
}
