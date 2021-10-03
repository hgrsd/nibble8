use crate::display::terminal_display::TerminalDisplay;
use crate::machine::chip8::Chip8;
use std::env;

mod display;
mod input;
mod machine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{} <rom.ch8>", args[0]);
        return;
    }
    let display = TerminalDisplay::new();
    let mut chip8 = Chip8::new(&display);
    chip8.load_rom(&args[1]);
    println!("{:?}", chip8);
}
