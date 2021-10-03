use crate::display::terminal_display::TerminalDisplay;
use crate::machine::chip8::Chip8;

mod machine;
mod display;
mod input;


fn main() {
    let display = TerminalDisplay::new();
    let chip8 = Chip8::new(&display);
    println!("{:?}", chip8);
}
