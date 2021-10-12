use display::sdl_display::SDLDisplay;
use machine::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::time::Duration;

mod bit_utils;
mod display;
mod machine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{} <rom.ch8>", args[0]);
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let mut display = SDLDisplay::init(&sdl_context, 640, 320);
    let mut chip8 = Chip8::new(&mut display);

    chip8.load_rom(&args[1]);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut iteration = 0;
    'running: loop {
        iteration += 1;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => chip8.register_key(0x00),
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => chip8.register_key(0x01),
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => chip8.register_key(0x02),
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => chip8.register_key(0x03),
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => chip8.register_key(0x04),
                Event::KeyDown {
                    keycode: Some(Keycode::Num5),
                    ..
                } => chip8.register_key(0x05),
                Event::KeyDown {
                    keycode: Some(Keycode::Num6),
                    ..
                } => chip8.register_key(0x06),
                Event::KeyDown {
                    keycode: Some(Keycode::Num7),
                    ..
                } => chip8.register_key(0x07),
                Event::KeyDown {
                    keycode: Some(Keycode::Num8),
                    ..
                } => chip8.register_key(0x08),
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => chip8.register_key(0x09),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => chip8.register_key(0x0A),
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => chip8.register_key(0x0B),
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => chip8.register_key(0x0C),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => chip8.register_key(0x0D),
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => chip8.register_key(0x0E),
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => chip8.register_key(0x0F),
                _ => {
                    if iteration > 40 {
                        chip8.clear_keys();
                        iteration = 0;
                    }
                }
            }
        }
        chip8.tick();
        std::thread::sleep(Duration::from_millis(1));
    }
}
