use display::sdl_display::SDLDisplay;
use input::Input;
use machine::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

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

    let input = Rc::new(RefCell::new(Input::new()));
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("nibble8", 320, 160)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut display = SDLDisplay::new(&mut canvas, 320, 160);
    let mut chip8 = Chip8::new(&mut display, input.clone());

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
                } => input.borrow_mut().register(0x00),
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => input.borrow_mut().register(0x01),
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => input.borrow_mut().register(0x02),
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => input.borrow_mut().register(0x03),
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => input.borrow_mut().register(0x04),
                Event::KeyDown {
                    keycode: Some(Keycode::Num5),
                    ..
                } => input.borrow_mut().register(0x05),
                Event::KeyDown {
                    keycode: Some(Keycode::Num6),
                    ..
                } => input.borrow_mut().register(0x06),
                Event::KeyDown {
                    keycode: Some(Keycode::Num7),
                    ..
                } => input.borrow_mut().register(0x07),
                Event::KeyDown {
                    keycode: Some(Keycode::Num8),
                    ..
                } => input.borrow_mut().register(0x08),
                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => input.borrow_mut().register(0x09),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => input.borrow_mut().register(0x0A),
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => input.borrow_mut().register(0x0B),
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => input.borrow_mut().register(0x0C),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => input.borrow_mut().register(0x0D),
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => input.borrow_mut().register(0x0E),
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => input.borrow_mut().register(0x0F),
                _ => {}
            }
        }
        chip8.tick();
        if iteration == 20 {
            input.borrow_mut().clear();
            iteration = 0;
        }
    }
}
