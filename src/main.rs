use std::fs;

use raylib::prelude::*;

use crate::arithmetic_peripheral::ArithmeticPeripheral;
use crate::peripherals_store::{PeripheralEvent, PeripheralsStore};
use crate::runtime::JsRuntime;

#[macro_use]
mod macros;

mod runtime;
mod peripherals_store;
mod peripheral;
mod arithmetic_peripheral;

const SCREEN_WIDTH: i32 = 192;
const SCREEN_HEIGHT: i32 = 128;
const WINDOW_SCALE: i32 = 6;

// const SCREEN_RECTANGLE: Rectangle = Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, -SCREEN_HEIGHT as f32);
// const VECTOR_ZERO: Vector2 = Vector2::new(0.0, 0.0);

fn read_script() -> String {
    fs::read_to_string("js/main.js").unwrap()
}

fn main() {
    println!("Creating runtime");

    let mut runtime = JsRuntime::new();

    let arithmetic_peripheral = Box::new(ArithmeticPeripheral::new());
    PeripheralsStore::mount(runtime.isolate(), String::from("ARITHMETIC"), arithmetic_peripheral);

    println!("Adding a dummy event");
    PeripheralsStore::push_event(&mut runtime.handle_scope(), PeripheralEvent {
        peripheral_id: String::from("DUMMY"),
        name: String::from("test"),
        data: None,
    });

    println!("Executing script");
    runtime.execute(read_script().as_str());
    println!("Executed script");

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH * WINDOW_SCALE, SCREEN_HEIGHT * WINDOW_SCALE)
        .title("retro-rust")
        .vsync()
        .build();

    let mut rl = &mut rl;

    // let mut window_rectangle = Rectangle::EMPTY;
    //
    // let screen_texture = &mut rl
    //     .load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
    //     .unwrap();


    // {
    //     let mut d = rl.begin_texture_mode(&thread, screen_texture);
    //
    //     d.clear_background(Color::BLACK);
    //
    //     d.draw_circle(10, 10, 5.0, Color::RED);
    //     d.draw_text("RenderTexture", 20, 20, 20, Color::ORANGE);
    //
    //     d.draw_rectangle_lines(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLUE);
    //
    //     for x in 0..SCREEN_WIDTH/2 {
    //         d.draw_line(x*2, 0, x*2, SCREEN_HEIGHT, Color::DARKBLUE);
    //     }
    //
    //     for y in 0..SCREEN_HEIGHT/2 {
    //         d.draw_line(0, y*2, SCREEN_WIDTH, y*2, Color::DARKBLUE);
    //     }
    //
    //     d.draw_line(0, 0, SCREEN_WIDTH, 0, Color::RED);
    //     d.draw_line(1, 0, 1, SCREEN_HEIGHT, Color::ORANGE);
    //
    //     d.draw_pixel(0, 0, Color::YELLOW);
    //     d.draw_pixel(0, SCREEN_HEIGHT-1, Color::YELLOW);
    //     d.draw_pixel(SCREEN_WIDTH-1, 0, Color::YELLOW);
    //     d.draw_pixel(SCREEN_WIDTH-1, SCREEN_HEIGHT-1, Color::YELLOW);
    // }

    while !rl.window_should_close() {
        let d = &mut rl.begin_drawing(&thread);

        //     window_rectangle.width = d.get_screen_width() as f32;
        //     window_rectangle.height = d.get_screen_height() as f32;

        d.clear_background(Color::WHITE);
        //     d.draw_texture_pro(&screen_texture, SCREEN_RECTANGLE, window_rectangle, VECTOR_ZERO, 0.0, Color::WHITE);
    }
}
