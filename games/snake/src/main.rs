use sdl2::{
    pixels::{self, Color},
    rect::Rect,
    video::WindowSurfaceRef,
};

static WIDTH: u32 = 900;
static HEIGHT: u32 = 600;

static GRID_SIZE: u32 = 15;
static ROWS: u32 = HEIGHT / GRID_SIZE;
static COLUMNS: u32 = WIDTH / GRID_SIZE;
static LINE_WIDTH: u32 = 2;

static GRID_COLOR: u32 = 0x1f1f1f1f;

fn argb_to_sdl_rbga(color: u32) -> Color {
    let a = ((color >> 24) & 0xff) as u8;
    let r = ((color >> 16) & 0xff) as u8;
    let g = ((color >> 8) & 0xff) as u8;
    let b = (color & 0xff) as u8;

    Color::RGBA(r, g, b, a)
}

fn draw_grid(surface: &mut WindowSurfaceRef<'_>) {
    let mut row_line = Rect::new(0, 0, WIDTH, LINE_WIDTH);

    while row_line.y < HEIGHT as i32 {
        surface
            .fill_rect(row_line, argb_to_sdl_rbga(GRID_COLOR))
            .unwrap();
        row_line.y += GRID_SIZE as i32;
    }

    let mut column_line = Rect::new(0, 0, LINE_WIDTH, HEIGHT);

    while column_line.x < WIDTH as i32 {
        surface
            .fill_rect(column_line, argb_to_sdl_rbga(GRID_COLOR))
            .unwrap();
        column_line.x += GRID_SIZE as i32;
    }
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = video_ctx
        .window("Classic Snake", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let event_pump = ctx.event_pump().unwrap();
    let mut surface = window.surface(&event_pump).unwrap();

    let rect = Rect::new(200, 200, 200, 200);
    surface.fill_rect(rect, pixels::Color::WHITE).unwrap();

    draw_grid(&mut surface);

    surface.update_window().unwrap();

    ctx.timer().unwrap().delay(5000);
}
