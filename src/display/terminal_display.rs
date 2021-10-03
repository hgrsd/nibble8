use crate::{bit_utils::get_bit_from_byte, display::chip8_display::Chip8Display};
use std::io::{Read, StdoutLock, Write};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct TerminalDisplay {
    cols: usize,
    rows: usize,
}

impl TerminalDisplay {
    pub fn new(cols: usize, rows: usize) -> TerminalDisplay {
        TerminalDisplay { cols, rows }
    }
}

impl Chip8Display for TerminalDisplay {
    fn draw(&mut self, bytes: &[u8]) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1)); // clear screen & move cursor to (1, 1)
        let mut col = 0;
        for byte in bytes {
            for bit in 0..8 {
                let is_on = get_bit_from_byte(bit, byte);
                print!(
                    "{}{}{}{}",
                    termion::color::Fg(termion::color::Green),
                    termion::color::Bg(termion::color::Black),
                    termion::cursor::Hide,
                    if is_on { "*" } else { " " }
                );
                col += 1;
                if col % self.cols == 0 {
                    print!("{}\r\n", termion::cursor::Hide);
                }
            }
        }
    }
}
