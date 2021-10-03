use crate::{bit_utils::get_bit_from_byte, display::chip8_display::Chip8Display};

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
        print!("\x1B[2J\x1B[1;1H"); // clear screen & move cursor to (0, 0)
        let mut col = 0;
        for byte in bytes {
            for bit in 0..8 {
                let is_on = get_bit_from_byte(bit, byte);
                print!("{}", if is_on { "\u{25AF}" } else { " " });
                col += 1;
                if col % self.cols == 0 {
                    println!();
                }
            }
        }
    }
}
