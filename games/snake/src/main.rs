use std::time::{Duration, Instant};

use rand::Rng;
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
    x: i32,
    y: i32,
    pnext: Option<Box<SnakeElement>>,
}

struct Direction {
    dx: i32,
    dy: i32,
}

struct Apple {
    x: i32,
    y: i32,
}

fn move_snake(snake: &mut SnakeElement, direction: &Direction) {
    snake.x += direction.dx;
    snake.y += direction.dy;
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

fn fill_cell(canvas: &mut Canvas<Window>, x: i32, y: i32, color: u32) {
    let cell_color = argb_to_sdl_color(color);
    let cell_rect = Rect::new(
        x * CELL_SIZE as i32,
        y * CELL_SIZE as i32,
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

fn reset_apple(snake: &SnakeElement, apple: &mut Apple) {
    let mut rng = rand::rng();

    apple.x = (COLUMNS as f64 * rng.random_range(0.0..=1.0)) as i32;
    apple.y = (ROWS as f64 * rng.random_range(0.0..=1.0)) as i32;

    let mut current_snake = snake;

    loop {
        if current_snake.x == apple.x && current_snake.y == apple.y {
            reset_apple(current_snake, apple);
            break;
        }

        if current_snake.pnext.is_none() {
            break;
        }

        current_snake = current_snake.pnext.as_ref().unwrap();
    }
}

struct Game {
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    snake: Box<SnakeElement>,
    apple: Apple,
    direction: Direction,
    last_frame: Instant,
}

impl emscripten_main_loop::MainLoop for Game {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        let frame_duration = Duration::from_millis(300);

        if self.last_frame.elapsed() < frame_duration {
            return emscripten_main_loop::MainLoopEvent::Continue;
        }

        self.last_frame = Instant::now();

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
                } => {
                    self.direction.dy = -1;
                    self.direction.dx = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.direction.dy = 1;
                    self.direction.dx = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.direction.dy = 0;
                    self.direction.dx = -1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.direction.dy = 0;
                    self.direction.dx = 1;
                }
                _ => {}
            }
        }

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        move_snake(&mut self.snake, &self.direction);

        if self.snake.x == self.apple.x && self.snake.y == self.apple.y {
            reset_apple(&self.snake, &mut self.apple);
        }

        println!("{} {}", self.apple.x, self.apple.y);

        fill_cell(&mut self.canvas, self.apple.x, self.apple.y, COLOR_APPLE);
        draw_snake(&mut self.canvas, Some(&self.snake));
        draw_grid(&mut self.canvas);

        self.canvas.present();

        emscripten_main_loop::MainLoopEvent::Continue
    }
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    let window = video
        .window("TisoOS", WIDTH, HEIGHT)
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

    let mut apple = Apple { x: 0, y: 0 };
    reset_apple(&snake, &mut apple);

    let direction = Direction { dx: 0, dy: 0 };

    let game = Game {
        canvas,
        event_pump,
        snake,
        direction,
        apple,
        last_frame: Instant::now(),
    };

    emscripten_main_loop::run(game);
}
