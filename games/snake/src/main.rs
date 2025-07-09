use sdl2::{
    Sdl, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

static BLACK: Color = Color::RGB(0, 0, 0);
static WHITE: Color = Color::RGB(255, 255, 255);

struct Game {
    pub ctx: Sdl,
    pub rect: Rect,
    pub canvas: Canvas<Window>,
}

impl emscripten_main_loop::MainLoop for Game {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        let mut events = self.ctx.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return emscripten_main_loop::MainLoopEvent::Terminate,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.rect.x -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.rect.x += 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.rect.y -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.rect.y += 10;
                }
                _ => {}
            }
        }

        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(WHITE);
        let _ = self.canvas.fill_rect(self.rect);
        self.canvas.present();

        emscripten_main_loop::MainLoopEvent::Continue
    }
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("Hello, Rust / SDL2 / WASM!", 640, 480)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {err}"),
    };

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {err}"),
    };

    let rect = Rect::new(0, 0, 10, 10);

    let game = Game { ctx, rect, canvas };

    emscripten_main_loop::run(game);
}
