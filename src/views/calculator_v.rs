use hypertext::{Raw, prelude::*};

pub fn render_calculator_file() -> impl Renderable {
    maud! {
        div data-file-type="calculator" class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-18 h-fit cursor-pointer file" {
            img class="w-9 h-9 select-none" src="/assets/images/calculator.svg" draggable="false";
            div class="text-white text-sm text-center select-none" {
                "Calculator"
            }
        }
    }
}

pub fn render_calculator_window(parent_height: i32, parent_width: i32) -> impl Renderable {
    let window_width = 320;
    let window_height = 480;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        (Raw::dangerously_create(
           r#"
                <script type="module">
                    import {setupCalculatorWindow, setupCalculatorToolBar} from "/assets/js/calculator.js";
                    setupCalculatorWindow();
                    setupCalculatorToolBar();
                </script>
            "#
        ))
        div
            id={ "calculator-window" }
            class="absolute flex flex-col bg-zinc-950 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={ "calculator-header" } class="flex justify-between items-center bg-zinc-900 px-3 h-12 select-none" {
                div class="flex items-center gap-2" {
                    img class="w-5 h-5" src="/assets/images/calculator.svg" draggable="false";
                    "Calculator"
                }
                div class="flex items-center gap-3" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            div class="flex flex-col gap-4 p-4 h-full" {
                div id={ "calculator-display" } class="flex justify-end items-center bg-zinc-800 p-4 rounded-sm h-20 overflow-hidden font-mono text-3xl text-right" {
                    "0"
                }
                div id={ "calculator-buttons" } class="flex-1 gap-2 grid grid-cols-4" {
                    button data-action="clear" class="col-span-2 bg-red-600 hover:bg-red-700 rounded-sm font-semibold text-xl" { "C" }
                    button data-action="delete" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "←" }
                    button data-action="operator" data-value="/" class="bg-orange-600 hover:bg-orange-700 rounded-sm text-xl" { "÷" }

                    button data-action="number" data-value="7" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "7" }
                    button data-action="number" data-value="8" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "8" }
                    button data-action="number" data-value="9" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "9" }
                    button data-action="operator" data-value="*" class="bg-orange-600 hover:bg-orange-700 rounded-sm text-xl" { "×" }

                    button data-action="number" data-value="4" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "4" }
                    button data-action="number" data-value="5" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "5" }
                    button data-action="number" data-value="6" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "6" }
                    button data-action="operator" data-value="-" class="bg-orange-600 hover:bg-orange-700 rounded-sm text-xl" { "−" }

                    button data-action="number" data-value="1" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "1" }
                    button data-action="number" data-value="2" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "2" }
                    button data-action="number" data-value="3" class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "3" }
                    button data-action="operator" data-value="+" class="bg-orange-600 hover:bg-orange-700 rounded-sm text-xl" { "+" }

                    button data-action="number" data-value="0" class="col-span-2 bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "0" }
                    button data-action="number" data-value="." class="bg-zinc-700 hover:bg-zinc-600 rounded-sm text-xl" { "." }
                    button data-action="equals" class="bg-green-600 hover:bg-green-700 rounded-sm text-xl" { "=" }
                }
            }
        }
    }
}
