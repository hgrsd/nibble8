use display::terminal_display::TerminalDisplay;
use input::Input;
use machine::chip8::{Chip8, DISPLAY_COLS, DISPLAY_ROWS};
use std::cell::RefCell;
use std::env;
use std::io::{Read, Write};
use std::rc::Rc;
use std::time::Duration;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod bit_utils;
mod display;
mod input;
mod machine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{} <rom.ch8>", args[0]);
        return;
    }
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    let mut display = TerminalDisplay::new(DISPLAY_COLS, DISPLAY_ROWS);
    let input = Rc::new(RefCell::new(Input::new()));
    let mut chip8 = Chip8::new(&mut display, input.clone());
    chip8.load_rom(&args[1]);

    let mut input_reset_cnt = 0;
    loop {
        if let Some(Ok(k)) = stdin.next() {
            input.borrow_mut().register(k);
        }
        chip8.tick();
        stdout.lock().flush().unwrap();
        input_reset_cnt += 1;
        if input_reset_cnt % 30 == 0 {
            input.borrow_mut().clear();
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
