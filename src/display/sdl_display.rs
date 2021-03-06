use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window, Sdl};

use crate::{
    bit_utils::get_bit_from_byte,
    machine::chip8::{DISPLAY_COLS, DISPLAY_ROWS},
};

use super::chip8_display::Chip8Display;

pub struct SDLDisplay {
    canvas: Canvas<Window>,
}

impl SDLDisplay {
    pub fn init(sdl_context: &Sdl, width: u32, height: u32) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("nibble8", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas
            .set_scale(
                (width / DISPLAY_COLS as u32) as f32,
                (height / DISPLAY_ROWS as u32) as f32,
            )
            .unwrap();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        SDLDisplay { canvas }
    }
}

impl Chip8Display for SDLDisplay {
    fn draw(&mut self, bytes: &[u8]) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(
                self.canvas.default_pixel_format(),
                DISPLAY_COLS as u32,
                DISPLAY_ROWS as u32,
            )
            .unwrap();
        self.canvas
            .with_texture_canvas(&mut texture, |canvas| {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                canvas.set_draw_color(Color::GREEN);
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
                            canvas.draw_point(Point::new(col as i32, row)).unwrap();
                        }
                        col += 1;
                    }
                }
            })
            .unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}
