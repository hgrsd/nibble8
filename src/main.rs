use display::terminal_display::TerminalDisplay;
use machine::chip8::{Chip8, DISPLAY_COLS, DISPLAY_ROWS};
use std::env;

mod display;
mod input;
mod machine;
mod bit_utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{} <rom.ch8>", args[0]);
        return;
    }
    let mut display = TerminalDisplay::new(DISPLAY_COLS, DISPLAY_ROWS);
    let mut chip8 = Chip8::new(&mut display);
    chip8.load_rom(&args[1]);
    chip8.run();
}
