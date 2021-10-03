use std::fmt::{Debug, Formatter};
use crate::display::virtual_display::VirtualDisplay;
use crate::machine::ram::Ram;
use crate::machine::registers::Registers;
use crate::machine::stack::Stack;

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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8<'a> {
    ram: Ram,
    registers: Registers,
    stack: Stack,
    display: &'a dyn VirtualDisplay,
}

impl <'a> Chip8<'a> {
    pub fn new(display: &'a dyn VirtualDisplay) -> Chip8 {
        let mut ram = Ram::initialise();
        // initialise the font sprites at 0 offset
        ram.write_bytes(0x000, &FONT_SPRITES);
        Chip8 {
            ram,
            registers: Registers::new(),
            stack: Stack::new(),
            display,
        }
    }
}

impl Debug for Chip8<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.registers).unwrap();
        write!(f, "{:?}", self.stack).unwrap();
        write!(f, "{:?}", self.ram)
    }
}