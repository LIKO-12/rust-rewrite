// use raylib::prelude::*;

// use crate::owned_texture_mode::OwnedTextureMode;

// mod graphics;
// mod owned_texture_mode;

// const SCREEN_WIDTH: i32 = 192;
// const SCREEN_HEIGHT: i32 = 128;
// const WINDOW_SCALE: i32 = 6;
//
// const SCREEN_RECTANGLE: Rectangle = Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, -SCREEN_HEIGHT as f32);
// const VECTOR_ZERO: Vector2 = Vector2::new(0.0, 0.0);

mod peripheral;
mod peripherals_store;

fn main() {
    // let (mut rl, thread) = raylib::init()
    //     .size(SCREEN_WIDTH * WINDOW_SCALE, SCREEN_HEIGHT * WINDOW_SCALE)
    //     .title("LIKO-12 • 2.0.0 • Development")
    //     .vsync()
    //     .build();
    //
    // let screen_texture = rl
    //     .load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
    //     .unwrap();
    //
    // let mut d = OwnedTextureMode::begin(rl, thread, screen_texture);
    //
    // d.clear_background(Color::BLACK);
    //
    // d.draw_circle(10, 10, 5.0, Color::RED);
    // d.draw_text("RenderTexture", 20, 20, 20, Color::ORANGE);
    //
    // d.draw_rectangle_lines(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLUE);
    //
    // for x in 0..SCREEN_WIDTH/2 {
    //     d.draw_line(x*2, 0, x*2, SCREEN_HEIGHT, Color::DARKBLUE);
    // }
    //
    // for y in 0..SCREEN_HEIGHT/2 {
    //     d.draw_line(0, y*2, SCREEN_WIDTH, y*2, Color::DARKBLUE);
    // }
    //
    // d.draw_line(0, 0, SCREEN_WIDTH, 0, Color::RED);
    // d.draw_line(1, 0, 1, SCREEN_HEIGHT, Color::ORANGE);
    //
    // d.draw_pixel(0, 0, Color::YELLOW);
    // d.draw_pixel(0, SCREEN_HEIGHT-1, Color::YELLOW);
    // d.draw_pixel(SCREEN_WIDTH-1, 0, Color::YELLOW);
    // d.draw_pixel(SCREEN_WIDTH-1, SCREEN_HEIGHT-1, Color::YELLOW);
    //
    //
    // let (mut rl, thread, screen_texture) = d.end();
    //
    // let mut window_rectangle = Rectangle::EMPTY;
    //
    // while !rl.window_should_close() {
    //     let d = &mut rl.begin_drawing(&thread);
    //
    //     window_rectangle.width = d.get_screen_width() as f32;
    //     window_rectangle.height = d.get_screen_height() as f32;
    //
    //     d.clear_background(Color::WHITE);
    //     d.draw_texture_pro(&screen_texture, SCREEN_RECTANGLE, window_rectangle, VECTOR_ZERO, 0.0, Color::WHITE);
    // }
}