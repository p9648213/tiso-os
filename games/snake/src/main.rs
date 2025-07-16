use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

static WIDTH: u32 = 900;
static HEIGHT: u32 = 600;

static GRID_SIZE: u32 = 15;
static ROWS: u32 = HEIGHT / GRID_SIZE;
static COLUMNS: u32 = WIDTH / GRID_SIZE;
static LINE_WIDTH: u32 = 2;

static GRID_COLOR: u32 = 0x1f1f1f1f;

fn argb_to_sdl_color(color: u32) -> Color {
    let a = ((color >> 24) & 0xff) as u8;
    let r = ((color >> 16) & 0xff) as u8;
    let g = ((color >> 8) & 0xff) as u8;
    let b = (color & 0xff) as u8;
    Color::RGBA(r, g, b, a)
}

fn draw_grid(canvas: &mut Canvas<Window>) {
    let grid_color = argb_to_sdl_color(GRID_COLOR);

    // Draw horizontal lines
    let mut row_line = Rect::new(0, 0, WIDTH, LINE_WIDTH);

    while row_line.y < HEIGHT as i32 {
        canvas.set_draw_color(grid_color);
        canvas.fill_rect(row_line).unwrap();
        row_line.y += GRID_SIZE as i32;
    }

    // Draw vertical lines
    let mut column_line = Rect::new(0, 0, LINE_WIDTH, HEIGHT);

    while column_line.x < WIDTH as i32 {
        canvas.set_draw_color(grid_color);
        canvas.fill_rect(column_line).unwrap();
        column_line.x += GRID_SIZE as i32;
    }
}

struct Game {
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    rect: Rect,
}

impl emscripten_main_loop::MainLoop for Game {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return emscripten_main_loop::MainLoopEvent::Terminate,
                _ => {}
            }
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.fill_rect(self.rect).unwrap();

        draw_grid(&mut self.canvas);

        self.canvas.present();

        emscripten_main_loop::MainLoopEvent::Continue
    }
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    let window = video
        .window("SDL2 Grid WASM", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().present_vsync().build().unwrap();
    let event_pump = ctx.event_pump().unwrap();

    let rect = Rect::new(200, 200, 200, 200);

    let game = Game {
        canvas,
        event_pump,
        rect,
    };

    emscripten_main_loop::run(game);
}
