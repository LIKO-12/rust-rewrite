use std::cell::{RefCell, RefMut};

use raylib::{RaylibHandle, RaylibThread};
use raylib::prelude::{RaylibDraw, RaylibTextureMode, RenderTexture2D, Color as RaylibColor, Vector2, Rectangle};

use crate::graphics::painter::*;
use crate::owned_texture_mode::OwnedTextureMode;

mod painter;

struct RayLibGraphics {
    /// The current active color.
    color: Color,

    palette: [RaylibColor; 16],

    /// The raylib texture mode drawing handle, which draws into the LIKO-12 screen.
    d: OwnedTextureMode,
}

impl RayLibGraphics {
    fn new(mut rl: RaylibHandle, thread: RaylibThread, width: u32, height: u32) -> Self {
        let screen_texture = rl
            .load_render_texture(&thread, width, height)
            .unwrap();

        let d = OwnedTextureMode::begin(rl, thread, screen_texture);

        // TODO: Load the palette from an embedded image file.

        let palette = [
            RaylibColor::get_color(0x000000), // 0
            RaylibColor::get_color(0x1c2b53), // 1
            RaylibColor::get_color(0x7f2454), // 2
            RaylibColor::get_color(0x008751), // 3
            RaylibColor::get_color(0xab5236), // 4
            RaylibColor::get_color(0x60584f), // 5
            RaylibColor::get_color(0xc3c3c6), // 6
            RaylibColor::get_color(0xfff1e9), // 7
            RaylibColor::get_color(0xed1b51), // 8
            RaylibColor::get_color(0xfaa21b), // 9
            RaylibColor::get_color(0xf7ec2f), // 10
            RaylibColor::get_color(0x5dbb4d), // 11
            RaylibColor::get_color(0x51a6dc), // 12
            RaylibColor::get_color(0x83769c), // 13
            RaylibColor::get_color(0xf176a6), // 14
            RaylibColor::get_color(0xfcccab), // 15
        ];

        RayLibGraphics {
            color: 0,
            palette,
            d,
        }
    }
}

impl GraphicsPainter for RayLibGraphics {
    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn clear(&mut self, color: Option<Color>) {
        let color = self.palette[color.unwrap_or(self.color) as usize];
        self.d.clear_background(color);
    }

    fn point(&mut self, x: f64, y: f64, color: Option<Color>) {
        let color = self.palette[color.unwrap_or(self.color) as usize];
        let v = Vector2::new(x as f32, y as f32);
        self.d.draw_pixel_v(v, color);
    }

    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Option<Color>) {
        let color = self.palette[color.unwrap_or(self.color) as usize];

        let start_pos = Vector2::new(x1 as f32, y1 as f32);
        let end_pos = Vector2::new(x2 as f32, y2 as f32);

        self.d.draw_line_v(start_pos, end_pos, color);
    }

    fn triangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, filled: bool, color: Option<Color>) {
        let color = self.palette[color.unwrap_or(self.color) as usize];

        let v1 = Vector2::new(x1 as f32, y1 as f32);
        let v2 = Vector2::new(x2 as f32, y2 as f32);
        let v3 = Vector2::new(x3 as f32, y3 as f32);

        match filled {
            true => self.d.draw_triangle(v1, v2, v3, color),
            false => self.d.draw_triangle_lines(v1, v2, v3, color),
        }
    }

    fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64, filled: bool, color: Option<Color>) {
        let color = self.palette[color.unwrap_or(self.color) as usize];

        let rect = Rectangle::new(x as f32, y as f32, width as f32, height as f32);

        match filled {
            true => self.d.draw_rectangle_rec(rect, color),
            false => self.d.draw_rectangle_lines_ex(rect, 1, color),
        }
    }

    fn polygon(&mut self, vertices: &[(f64, f64)], color: Option<Color>) {
        todo!()
    }

    fn circle(&mut self, cx: f32, cy: f64, radius: f64, filled: bool, color: Option<Color>) {
        let color = self.palette[color.unwrap_or(self.color) as usize];

        let center = Vector2::new(cx as f32, cy as f32);

        match filled {
            true => self.d.draw_circle_v(center, radius as f32, color),
            false => self.d.draw_circle_sector_lines(center, radius as f32, 0, 360, radius as i32 / 2, color), // TODO: verify this works
        }
    }

    fn ellipse(&mut self, x: f32, y: f32, radius_x: f32, radius_y: f32, filled: bool, color: Option<Color>) {
        todo!()
    }
}
