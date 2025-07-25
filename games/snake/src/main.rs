use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

static WIDTH: u32 = 900;
static HEIGHT: u32 = 600;

static CELL_SIZE: u32 = 30;
static ROWS: u32 = HEIGHT / CELL_SIZE;
static COLUMNS: u32 = WIDTH / CELL_SIZE;
static LINE_WIDTH: u32 = 2;

static COLOR_GRID: u32 = 0x1f1f1f1f;
static COLOR_WHITE: u32 = 0xffffffff;
static COLOR_APPLE: u32 = 0x00ff0000;

struct SnakeElement {
    x: u32,
    y: u32,
    pnext: Option<Box<SnakeElement>>,
}

fn argb_to_sdl_color(color: u32) -> Color {
    let a = ((color >> 24) & 0xff) as u8;
    let r = ((color >> 16) & 0xff) as u8;
    let g = ((color >> 8) & 0xff) as u8;
    let b = (color & 0xff) as u8;
    Color::RGBA(r, g, b, a)
}

fn draw_grid(canvas: &mut Canvas<Window>) {
    let grid_color = argb_to_sdl_color(COLOR_GRID);

    // Draw horizontal lines
    let mut row_line = Rect::new(0, 0, WIDTH, LINE_WIDTH);

    while row_line.y < HEIGHT as i32 {
        canvas.set_draw_color(grid_color);
        canvas.fill_rect(row_line).unwrap();
        row_line.y += CELL_SIZE as i32;
    }

    // Draw vertical lines
    let mut column_line = Rect::new(0, 0, LINE_WIDTH, HEIGHT);

    while column_line.x < WIDTH as i32 {
        canvas.set_draw_color(grid_color);
        canvas.fill_rect(column_line).unwrap();
        column_line.x += CELL_SIZE as i32;
    }
}

fn fill_cell(canvas: &mut Canvas<Window>, x: u32, y: u32, color: u32) {
    let cell_color = argb_to_sdl_color(color);
    let cell_rect = Rect::new(
        (x * CELL_SIZE) as i32,
        (y * CELL_SIZE) as i32,
        CELL_SIZE,
        CELL_SIZE,
    );
    canvas.set_draw_color(cell_color);
    canvas.fill_rect(cell_rect).unwrap();
}

fn draw_snake(canvas: &mut Canvas<Window>, snake: Option<&SnakeElement>) {
    if let Some(snake) = snake {
        fill_cell(canvas, snake.x, snake.y, COLOR_WHITE);
        draw_snake(canvas, snake.pnext.as_deref());
    }
}

struct Game {
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    snake: Box<SnakeElement>,
    apple_x: u32,
    apple_y: u32,
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
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.snake.y -= 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => self.snake.y += 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.snake.x -= 1,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.snake.x += 1,
                _ => {}
            }
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        draw_snake(&mut self.canvas, Some(&self.snake));
        fill_cell(&mut self.canvas, self.apple_x, self.apple_y, COLOR_APPLE);
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

    let snake = Box::new(SnakeElement {
        x: 5,
        y: 5,
        pnext: None,
    });

    let game = Game {
        canvas,
        event_pump,
        snake,
        apple_x: 13,
        apple_y: 17,
    };

    emscripten_main_loop::run(game);
}
