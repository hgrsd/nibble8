use crate::display::virtual_display::VirtualDisplay;
use crate::machine::ram::{Ram, MAX_SIZE};
use crate::machine::registers::Registers;
use crate::machine::stack::Stack;
use std::fmt::{Debug, Formatter};
use std::fs::read;

// 16 default font sprites; each sprite is 5 bytes long (8*5 pixels)
const FONT_SPRITES: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const PROGRAM_OFFSET: usize = 0x200; // offset at which the start of a program should be loaded

pub struct Chip8<'a> {
    ram: Ram,
    program_counter: usize,
    registers: Registers,
    stack: Stack,
    display: &'a dyn VirtualDisplay,
}

impl<'a> Chip8<'a> {
    pub fn new(display: &'a dyn VirtualDisplay) -> Chip8 {
        let mut ram = Ram::initialise();
        ram.write_bytes(0x000, &FONT_SPRITES);
        Chip8 {
            program_counter: 0,
            ram,
            registers: Registers::new(),
            stack: Stack::new(),
            display,
        }
    }

    pub fn load_rom(&mut self, file: &str) {
        let bytes = read(file).expect("Unable to read file");
        self.ram.write_bytes(PROGRAM_OFFSET, &bytes);
    }
}

impl Debug for Chip8<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.registers).unwrap();
        write!(f, "{:?}", self.stack).unwrap();
        write!(f, "{:?}", self.ram)
    }
}
