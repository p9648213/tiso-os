use hypertext::{
    GlobalAttributes, HtmxAttributes, Raw, Renderable, html_elements, maud, maud_move,
};

use crate::{contanst::MIN_RECTANGLE_WIDTH, controllers::account_c::AccountForm};

pub enum ItemType {
    Text,
    Folder,
}

pub fn render_welcome_screen() -> impl Renderable {
    let account_form = AccountForm {
        username: "".to_string(),
        password: "".to_string(),
        confirm_password: None,
    };

    maud_move!(
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                link rel="stylesheet" href="/assets/css/lib/tailwind.css";
                link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico";
                script src="/assets/js/lib/htmx.js" defer {}
                script src="/assets/js/main.js" type="module" defer {}
            }
            title { "TisoOS - Welcome" }
            body style="background: radial-gradient(ellipse at top left, #070f2b, #1b1a55, #535c91);" class="relative overflow-hidden" {
                main class="relative flex flex-col justify-center items-center min-h-screen text-white text-center" {
                    div class="mb-8" {
                        div class="flex justify-center items-center bg-white bg-opacity-20 backdrop-blur-sm mx-auto mb-6 rounded-2xl w-24 h-24" {
                            div class="flex justify-center items-center bg-white rounded-lg w-12 h-12" {
                                div class="bg-blue-600 rounded-sm w-6 h-6" {}
                            }
                        }
                        h1 class="mb-2 font-light text-4xl" { "TisoOS" }
                        p class="font-light text-lg" { "Your gateway to productivity" }
                    }
                    div class="mb-8 max-w-md" {
                        h2 class="mb-4 font-light text-3xl" { "Welcome" }
                        p class="text-lg leading-relaxed" {
                            "Let's get you set up and ready to go. This will only take a few minutes."
                        }
                    }
                    (render_account_form(&account_form, true))
                }
            }
        }
    )
}

pub fn render_account_form(account_form: &AccountForm, register_mode: bool) -> impl Renderable {
    maud_move! {
        form
            id="account_form"
            hx-post="/action/create-account"
            hx-target="body"
            hx-trigger="submit"
            hx-indicator="#account_button"
            class="relative flex flex-col space-y-2 text-black"
        {

            input
                class="bg-white px-3 rounded-sm h-8"
                id="account_username"
                name="username"
                placeholder="Username"
                value=(account_form.username)
                hx-post="/action/create-account"
                hx-swap=(if register_mode { "outerHTML" } else { "none" })
                hx-target=(if register_mode { "#account_confirm_password" } else { "" })
                hx-trigger="input delay:300ms"
                hx-indicator="#account_button";
            input
                class="bg-white px-3 rounded-sm h-8"
                name="password" placeholder="Password"
                type="password"
                value=(account_form.password);
            @if register_mode {
                (render_comfirm_password(&account_form.confirm_password))
            }

            button id="account_button" type="submit" class="bg-white hover:opacity-90 rounded-sm h-8 cursor-pointer" { "Sign in" }
            div id="account_error" class="-bottom-5 absolute text-red-400" {}
        }
    }
}

pub fn render_comfirm_password(value: &Option<String>) -> impl Renderable {
    maud_move! {
        input
            class="bg-white px-3 rounded-sm h-8"
            name="confirm_password"
            id="account_confirm_password"
            placeholder="Confirm Password"
            type="password"
            value=(value)
            hx-post="/action/create-account"
            hx-target="#account_error"
            hx-trigger="input delay:300ms"
            hx-indicator="#account_button";
    }
}

pub fn render_screen() -> impl Renderable {
    maud! {
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                link rel="stylesheet" href="/assets/css/lib/tailwind.css";
                link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico";
                script src="/assets/js/lib/htmx.js" defer {}
                script src="/assets/js/main.js" type="module" defer {}
            }
            title { "TisoOS" }
            body style="background: radial-gradient(ellipse at top left, #070f2b, #1b1a55, #535c91);" class="relative overflow-hidden" {
                (render_screen_section())
            }
        }
    }
}

pub fn render_screen_section() -> impl Renderable {
    maud! {
        (Raw(r#"
            <script type="module">
                import {setupRightClickContextMenu} from "/assets/js/right_click.js";
                import {setupGridDimensions} from "/assets/js/grid.js";
                import {setupResize} from "/assets/js/resize.js";
                setupGridDimensions();
                setupResize();
                setupRightClickContextMenu();
            </script>
        "#))
        main class="flex flex-wrap h-[calc(100%-theme('spacing.12'))]" {}
        footer class="right-0 bottom-0 left-0 absolute bg-zinc-800 border-t border-t-zinc-700 h-12" {}
    }
}

pub fn render_screen_grid(height: u16, width: u16) -> impl Renderable {
    let rows = height / MIN_RECTANGLE_WIDTH;
    let cols = width / MIN_RECTANGLE_WIDTH;
    let rectangle_width = width as f32 / cols as f32 - 0.1;

    maud_move! {
        input id="screen_rows" type="hidden" value=(rows);
        input id="screen_cols" type="hidden" value=(cols);

        @for row in 0..rows {
            @for col in 0..cols {
                div class = "flex justify-center items-center relative"
                    style={ "width:" (rectangle_width) "px;" }
                    id={ "item-" (row) "-" (col) } {}
            }
        }
    }
}

pub fn render_screen_item(item_type: ItemType) -> impl Renderable {
    maud_move! {
        div class="absolute inset-0 flex justify-center items-center" {
            div class="flex flex-col justify-center items-center gap-1 hover:bg-blue-900 p-1.5 rounded-xs w-fit" {
                @match item_type {
                    ItemType::Text => {
                        img class="w-9 h-9" src="/assets/images/text-editor.svg";
                        div class="max-w-20 overflow-ellipsis overflow-hidden text-white text-sm whitespace-nowrap select-none" {
                            "New Text.txt"
                        }
                    }
                    ItemType::Folder => {
                        img class="w-9 h-9" src="/assets/images/folder.svg";
                        div class="max-w-20 overflow-ellipsis overflow-hidden text-white text-sm whitespace-nowrap select-none" {
                            "New Folder"
                        }
                    }
                }
            }
        }
    }
}
