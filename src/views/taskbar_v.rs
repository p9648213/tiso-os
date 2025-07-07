use hypertext::{GlobalAttributes, Raw, Renderable, html_elements, maud_move};

pub fn render_taskbar() -> impl Renderable {
    maud_move! {
      (Raw(r#"
          <script type="module">
              import {setupClock} from "/assets/js/clock.js";
              setupClock();
          </script>
      "#))
      footer class="right-0 bottom-0 left-0 absolute flex justify-between items-center bg-zinc-800 px-6 border-t border-t-zinc-700 h-12 text-white" {
        div {
          img src="/assets/images/menu.svg" alt="menu" class="hover:opacity-80 w-6.5 h-6.5 cursor-pointer" draggable="false";
        }
        div class="flex flex-col justify-center items-center h-full text-sm" {
          div id="clock-time" {}
          div id="clock-date" {}
        }
      }
    }
}
