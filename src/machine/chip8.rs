use crate::bit_utils::get_bit_from_byte;
use crate::display::virtual_display::VirtualDisplay;
use crate::machine::display_state::DisplayState;
use crate::machine::instruction::Instruction;
use crate::machine::ram::Ram;
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

pub const DISPLAY_COLS: usize = 64;
pub const DISPLAY_ROWS: usize = 32;

pub struct Chip8<'a> {
    ram: Ram,
    program_counter: usize,
    registers: Registers,
    stack: Stack,
    display: &'a mut dyn VirtualDisplay,
    display_state: DisplayState,
}

impl<'a> Chip8<'a> {
    pub fn new(display: &'a mut dyn VirtualDisplay) -> Chip8 {
        let mut ram = Ram::initialise();
        ram.write_bytes(0x000, &FONT_SPRITES);
        Chip8 {
            program_counter: 0x000,
            ram,
            registers: Registers::new(),
            stack: Stack::new(),
            display,
            display_state: DisplayState::new(DISPLAY_COLS, DISPLAY_ROWS),
        }
    }

    pub fn load_rom(&mut self, file: &str) {
        let bytes = read(file).expect("Unable to read file");
        self.ram.write_bytes(PROGRAM_OFFSET, &bytes);
        self.program_counter = PROGRAM_OFFSET;
    }

    fn load_sprite(&mut self, x: u8, y: u8, n_rows: u8) {
        // Chip-8 wraps around the starting coordinates for a sprite if they exceed the grid size.
        let wrapped_x = x as usize % DISPLAY_COLS;
        let wrapped_y = y as usize % DISPLAY_ROWS;

        self.registers.write_vx(0x0F, 0);
        let sprite_offset = self.registers.read_i();
        for row in 0..n_rows as usize {
            if wrapped_y + row == DISPLAY_ROWS {
                break;
            }
            let current_byte = self
                .ram
                .read_bytes(sprite_offset as usize + row as usize, 1)[0];
            for col in 0..8 {
                if wrapped_x + col > DISPLAY_COLS {
                    continue;
                }
                let grid_x = wrapped_x + col;
                let grid_y = wrapped_y + row;

                let new_state = get_bit_from_byte(col, current_byte);
                if !new_state {
                    continue;
                }

                let current_state = self.display_state.is_on(grid_x, grid_y);
                if current_state {
                    self.registers.write_vx(0x0F, 1);
                }
                self.display_state.flip(grid_x, grid_y);
            }
        }
        self.display.draw(&self.display_state.as_bytes());
    }

    pub fn run(&mut self) {
        loop {
            let next_instruction: Instruction = self.ram.read_bytes(self.program_counter, 2).into();
            self.program_counter += 0x002;
            match next_instruction {
                Instruction::_00E0 => {
                    self.display_state.clear();
                }
                Instruction::_00EE => {
                    self.program_counter = self.stack.pop() as usize;
                }
                Instruction::_1nnn(addr) => {
                    self.program_counter = addr as usize;
                }
                Instruction::_2nnn(addr) => {
                    self.stack.push(self.program_counter as u16);
                    self.program_counter = addr as usize;
                }
                Instruction::_6xkk(register, value) => {
                    self.registers.write_vx(register as usize, value);
                }
                Instruction::_7xkk(register, value) => {
                    let current_value = self.registers.read_vx(register as usize);
                    self.registers
                        .write_vx(register as usize, current_value + value);
                }
                Instruction::_Annn(addr) => {
                    self.registers.write_i(addr);
                }
                Instruction::_Dxyn(reg_x, reg_y, n_rows) => {
                    let x = self.registers.read_vx(reg_x as usize);
                    let y = self.registers.read_vx(reg_y as usize);
                    self.load_sprite(x, y, n_rows);
                }
                _ => unimplemented!(),
            }
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
