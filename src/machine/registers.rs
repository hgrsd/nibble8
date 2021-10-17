use std::fmt::{Debug, Formatter};

pub struct Registers {
    vx: [u8; 16],
    i: usize,
    delay: u8,
    sound: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            vx: [0x00; 16],
            i: 0x0000,
            delay: 0x00,
            sound: 0x00,
        }
    }

    pub fn read_vx(&self, x: u8) -> u8 {
        self.vx[x as usize]
    }

    pub fn write_vx(&mut self, x: u8, byte: u8) {
        self.vx[x as usize] = byte;
    }

    pub fn read_i(&self) -> usize {
        self.i
    }

    pub fn write_i(&mut self, bytes: usize) {
        self.i = bytes;
    }

    pub fn read_sound_timer(&self) -> u8 {
        self.sound
    }

    pub fn set_sound_timer(&mut self, byte: u8) {
        self.sound = byte;
    }

    pub fn read_delay_timer(&self) -> u8 {
        self.delay
    }

    pub fn set_delay_timer(&mut self, byte: u8) {
        self.delay = byte;
    }
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (register, value) in self.vx.iter().enumerate() {
            writeln!(f, "V{:X}: {:#04X}", register, value).unwrap();
        }
        writeln!(f, "I: {:#06X} ", self.i).unwrap();
        writeln!(f, "delay: {:#04X}", self.delay).unwrap();
        writeln!(f, "sound: {:#04X}", self.sound)
    }
}
