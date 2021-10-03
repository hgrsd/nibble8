use std::io::Error;
use std::time::Duration;
use termion::event::Key;

pub struct Input {
    pub current_key: Option<u8>,
}

impl Input {
    pub fn new() -> Self {
        Input { current_key: None }
    }

    pub fn clear(&mut self) {
        self.current_key = None;
    }

    pub fn register(&mut self, key: Key) {
        match key {
            termion::event::Key::Char('0') => {
                self.current_key = Some(0x00);
            }
            termion::event::Key::Char('1') => {
                self.current_key = Some(0x01);
            }
            termion::event::Key::Char('2') => {
                self.current_key = Some(0x02);
            }
            termion::event::Key::Char('3') => {
                self.current_key = Some(0x03);
            }
            termion::event::Key::Char('4') => {
                self.current_key = Some(0x04);
            }
            termion::event::Key::Char('5') => {
                self.current_key = Some(0x05);
            }
            termion::event::Key::Char('6') => {
                self.current_key = Some(0x06);
            }
            termion::event::Key::Char('7') => {
                self.current_key = Some(0x07);
            }
            termion::event::Key::Char('8') => {
                self.current_key = Some(0x08);
            }
            termion::event::Key::Char('9') => {
                self.current_key = Some(0x09);
            }
            termion::event::Key::Char('a') => {
                self.current_key = Some(0x0A);
            }
            termion::event::Key::Char('b') => {
                self.current_key = Some(0x0B);
            }
            termion::event::Key::Char('c') => {
                self.current_key = Some(0x0C);
            }
            termion::event::Key::Char('d') => {
                self.current_key = Some(0x0D);
            }
            termion::event::Key::Char('e') => {
                self.current_key = Some(0x0E);
            }
            termion::event::Key::Char('f') => {
                self.current_key = Some(0x0F);
            }
            _ => {}
        }
    }
}
