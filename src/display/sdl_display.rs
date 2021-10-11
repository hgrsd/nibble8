use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{bit_utils::get_bit_from_byte, machine::chip8::DISPLAY_COLS};

use super::chip8_display::Chip8Display;

pub struct SDLDisplay<'a> {
    canvas: &'a mut Canvas<Window>,
}

impl<'a> SDLDisplay<'a> {
    pub fn new(canvas: &'a mut Canvas<Window>) -> Self {
        SDLDisplay { canvas }
    }
}

impl Chip8Display for SDLDisplay<'_> {
    fn draw(&mut self, bytes: &[u8]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::GREEN);

        let mut row = 0;
        let mut col = 0;
        for byte in bytes {
            for bit in 0..8 {
                if col == DISPLAY_COLS {
                    row += 1;
                    col = 0;
                }
                let is_on = get_bit_from_byte(bit, byte);
                if is_on {
                    self.canvas.draw_point(Point::new(col as i32, row)).unwrap();
                }
                col += 1;
            }
        }
        self.canvas.present();
    }
}
