use crate::bit_utils::get_bit_from_byte;
use crate::display::chip8_display::Chip8Display;
use crate::machine::display_state::DisplayState;
use crate::machine::instruction::Instruction;
use crate::machine::ram::Ram;
use crate::machine::registers::Registers;
use crate::machine::stack::Stack;
use rand::Rng;
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
    display: &'a mut dyn Chip8Display,
    display_state: DisplayState,
    tick: u8,
}

impl<'a> Chip8<'a> {
    pub fn new(display: &'a mut dyn Chip8Display) -> Chip8 {
        let mut ram = Ram::initialise();
        ram.write_bytes(0x000, &FONT_SPRITES);
        Chip8 {
            program_counter: 0x000,
            ram,
            registers: Registers::new(),
            stack: Stack::new(),
            display,
            display_state: DisplayState::new(DISPLAY_COLS, DISPLAY_ROWS),
            tick: 0,
        }
    }

    pub fn load_rom(&mut self, file: &str) {
        let bytes = read(file).expect("Unable to read file");
        self.ram.write_bytes(PROGRAM_OFFSET, &bytes);
        self.program_counter = PROGRAM_OFFSET;
    }

    pub fn run(&mut self) {
        loop {
            self.tick += 1;
            if self.tick % 10 == 0 {
                self.decr_timers();
                self.tick = 0;
            }
            self.tick();
        }
    }

    fn decr_timers(&mut self) {
        let delay = self.registers.read_delay_timer();
        if delay > 0 {
            self.registers.set_delay_timer(delay - 1);
        }

        let sound = self.registers.read_sound_timer();
        if sound > 0 {
            self.registers.set_sound_timer(sound - 1);
        }
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

                let new_state = get_bit_from_byte(col, &current_byte);
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

    fn tick(&mut self) {
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
            Instruction::_3xkk(register, value) => {
                if self.registers.read_vx(register as usize) == value {
                    self.program_counter += 2;
                }
            }
            Instruction::_4xkk(register, value) => {
                if self.registers.read_vx(register as usize) != value {
                    self.program_counter += 2;
                }
            }
            Instruction::_5xy0(reg_x, reg_y) => {
                if self.registers.read_vx(reg_x as usize) == self.registers.read_vx(reg_y as usize)
                {
                    self.program_counter += 2;
                }
            }
            Instruction::_6xkk(register, value) => {
                self.registers.write_vx(register as usize, value);
            }
            Instruction::_7xkk(register, value) => {
                let current_value = self.registers.read_vx(register as usize);
                self.registers.write_vx(
                    register as usize,
                    (current_value as u16 + value as u16) as u8,
                );
            }
            Instruction::_8xy0(reg_x, reg_y) => {
                self.registers
                    .write_vx(reg_x as usize, self.registers.read_vx(reg_y as usize));
            }
            Instruction::_8xy1(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                self.registers.write_vx(reg_x as usize, x | y);
            }
            Instruction::_8xy2(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                self.registers.write_vx(reg_x as usize, x & y);
            }
            Instruction::_8xy3(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                self.registers.write_vx(reg_x as usize, x ^ y);
            }
            Instruction::_8xy4(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize) as u16;
                let y = self.registers.read_vx(reg_y as usize) as u16;
                let added = x + y;
                self.registers
                    .write_vx(0x0F, if added > u8::MAX.into() { 1 } else { 0 });
                self.registers.write_vx(reg_x as usize, added as u8);
            }
            Instruction::_8xy5(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize) as u16;
                let y = self.registers.read_vx(reg_y as usize) as u16;
                self.registers.write_vx(0x0F, if x > y { 1 } else { 0 });
                let subtracted = if x > y { x - y } else { 0 };
                self.registers.write_vx(reg_x as usize, subtracted as u8);
            }
            Instruction::_8xy6(reg_x, _reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                self.registers
                    .write_vx(0x0F, if get_bit_from_byte(7, &x) { 1 } else { 0 });
                self.registers.write_vx(reg_x as usize, x >> 1);
            }
            Instruction::_8xy7(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                let subtracted = if x > y { y - x } else { 0 };
                self.registers.write_vx(0x0F, if y > x { 1 } else { 0 });
                self.registers.write_vx(reg_x as usize, subtracted);
            }
            Instruction::_8xyE(reg_x, _reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                self.registers
                    .write_vx(0x0F, if get_bit_from_byte(7, &x) { 1 } else { 0 });
                self.registers.write_vx(reg_x as usize, x << 1);
            }
            Instruction::_9xy0(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                if x != y {
                    self.program_counter += 2;
                }
            }
            Instruction::_Annn(addr) => {
                self.registers.write_i(addr);
            }
            Instruction::_Bnnn(addr) => {
                let v0 = self.registers.read_vx(0x00);
                self.program_counter += addr as usize + v0 as usize;
            }
            Instruction::_Dxyn(reg_x, reg_y, n_rows) => {
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                self.load_sprite(x, y, n_rows);
            }
            Instruction::_Cxkk(register, value) => {
                let rnd = rand::thread_rng().gen_range(0..=255) as u8;
                self.registers.write_vx(register as usize, rnd & value);
            }
            Instruction::_Ex9E(_) => todo!(),
            Instruction::_ExA1(_) => todo!(),
            Instruction::_Fx07(register) => {
                self.registers
                    .write_vx(register as usize, self.registers.read_delay_timer());
            }
            Instruction::_Fx0A(_) => todo!(),
            Instruction::_Fx15(register) => {
                self.registers
                    .set_delay_timer(self.registers.read_vx(register as usize));
            }
            Instruction::_Fx18(register) => {
                self.registers
                    .set_sound_timer(self.registers.read_vx(register as usize));
            }
            Instruction::_Fx1E(register) => {
                // TODO - do we need to guard against overflows?
                self.registers.write_i(
                    self.registers.read_i() + self.registers.read_vx(register as usize) as u16,
                );
            }
            Instruction::_Fx29(register) => {
                let byte = self.registers.read_vx(register as usize);
                let address = 0x000 + (byte * 5);
                self.registers.write_i(address as u16);
            }
            Instruction::_Fx33(register) => {
                let number = self.registers.read_vx(register as usize);
                let addr = self.registers.read_i();
                self.ram.write_bytes(addr as usize, &[number / 100]);
                self.ram
                    .write_bytes(addr as usize + 1, &[number % 100 / 10]);
                self.ram.write_bytes(addr as usize + 2, &[number % 10]);
            }
            Instruction::_Fx55(last_register) => {
                let addr = self.registers.read_i() as usize;
                for i in 0..=last_register as usize {
                    self.ram.write_bytes(addr + i, &[self.registers.read_vx(i)]);
                }
            }
            Instruction::_Fx65(last_register) => {
                let addr = self.registers.read_i() as usize;
                for i in 0..=last_register as usize {
                    let bytes = self.ram.read_bytes(addr + i, 1);
                    self.registers.write_vx(i, bytes[0]);
                }
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
