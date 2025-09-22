use hypertext::{Raw, prelude::*};

use crate::{
    models::file_db::{File, FileType},
    views::{
        calculator_v::render_calculator_file, flappy_bird_v::render_flappy_bird_file,
        snake_v::render_snake_file,
    },
};

pub fn render_taskbar() -> impl Renderable {
    maud! {
      (Raw::dangerously_create(r#"
          <script type="module">
              import {setupClock} from "/assets/js/clock.js";
              import {setupTaskbarMenuToggle} from "/assets/js/taskbar.js";
              setupTaskbarMenuToggle();
              setupClock();
          </script>
      "#))
      footer class="right-0 bottom-0 left-0 absolute flex justify-between items-center bg-zinc-800 px-6 border-t border-t-zinc-700 h-12 text-white" {
        div class="flex items-center gap-4" {
          div id="taskbar-menu-icon" {
            img src="/assets/images/menu.svg" alt="menu" class="hover:opacity-80 w-6.5 h-6.5 cursor-pointer" draggable="false";
          }
          div class="relative" {
            input type="test" class="bg-zinc-700 pr-3 pl-10 rounded-2xl w-50 h-8";
            img src="/assets/images/search.svg" alt="search" class="top-1 left-3 absolute w-6 h-6" draggable="false";
          }
          div id="taskbar-minimize" class="flex gap-3" {}
        }

        div class="flex flex-col justify-center items-center h-full text-sm" {
          div id="clock-time" {}
          div id="clock-date" {}
        }
      }
      div id="taskbar-menu" class="hidden bottom-14 left-2 absolute bg-zinc-800 p-3 rounded-sm w-100 h-100" {
        div
          hx-get="/read/taskbar/files"
          hx-trigger="load"
          id="taskbar-menu-files"
          class="flex gap-2" {}
      }
    }
}

pub fn render_taskbar_menu_files(files: &Vec<File>) -> impl Renderable {
    maud!(
      (Raw::dangerously_create(r#"
          <script type="module">
              import {setupTaskbarMenuFiles} from "/assets/js/taskbar.js";
              setupTaskbarMenuFiles();
          </script>
      "#))
      @for file in files {
        @match file.file_type {
          Some(FileType::Calculator) => {(render_calculator_file())},
          Some(FileType::Snake) => {(render_snake_file())},
          Some(FileType::FlappyBird) => {(render_flappy_bird_file())},
          Some(FileType::Txt) => {},
          Some(FileType::ThisPC) => {},
          None => {},
        }
      }
    )
}
