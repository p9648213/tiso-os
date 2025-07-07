use hypertext::{GlobalAttributes, Renderable, html_elements, maud_move};

pub fn render_taskbar() -> impl Renderable {
    maud_move! {
      footer class="right-0 bottom-0 left-0 absolute flex items-center bg-zinc-800 px-6 border-t border-t-zinc-700 h-12" {
        img src="/assets/images/menu.svg" alt="menu" class="hover:opacity-80 w-6.5 h-6.5 cursor-pointer" draggable="false";
      }
    }
}
