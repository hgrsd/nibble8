use crate::bit_utils::get_bit_from_byte;
use crate::display::chip8_display::Chip8Display;
use crate::machine::display_state::DisplayState;
use crate::machine::instruction::Instruction;
use crate::machine::ram::Ram;
use crate::machine::registers::Registers;
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
    stack: Vec<u16>,
    display: &'a mut dyn Chip8Display,
    display_state: DisplayState,
    tick: u8,
    keys_state: [bool; 16],
    current_key: Option<u8>,
}

impl<'a> Chip8<'a> {
    pub fn new(display: &'a mut dyn Chip8Display) -> Chip8<'a> {
        let mut ram = Ram::initialise();
        ram.write_bytes(0x000, &FONT_SPRITES);
        Chip8 {
            program_counter: 0x000,
            ram,
            registers: Registers::new(),
            stack: Vec::with_capacity(16),
            display,
            display_state: DisplayState::new(DISPLAY_COLS, DISPLAY_ROWS),
            tick: 0,
            keys_state: [false; 16],
            current_key: None,
        }
    }

    pub fn load_rom(&mut self, file: &str) {
        let bytes = read(file).expect("Unable to read file");
        self.ram.write_bytes(PROGRAM_OFFSET, &bytes);
        self.program_counter = PROGRAM_OFFSET;
    }

    pub fn register_key(&mut self, key: u8) {
        self.current_key = Some(key);
        self.keys_state[key as usize] = true;
    }

    pub fn clear_keys(&mut self) {
        self.keys_state = [false; 16];
        self.current_key = None;
    }

    fn is_pressed(&self, key: u8) -> bool {
        self.keys_state[key as usize]
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

        'bytes: for row in 0..n_rows as usize {
            let current_y = wrapped_y + row;
            if current_y >= DISPLAY_ROWS {
                continue;
            }
            let current_byte = self
                .ram
                .read_bytes(sprite_offset as usize + row as usize, 1)[0];
            for col in 0..8 {
                let current_x = wrapped_x + col;
                if current_x >= DISPLAY_COLS {
                    continue 'bytes;
                }

                let current_state = self.display_state.is_on(current_x, current_y);
                let new_state = get_bit_from_byte(col, &current_byte);

                match (current_state, new_state) {
                    (true, true) => {
                        self.registers.write_vx(0x0F, 1);
                        self.display_state.flip(current_x, current_y);
                    }
                    (false, true) => {
                        self.display_state.flip(current_x, current_y);
                    }
                    _ => {}
                }
            }
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        self.program_counter += 0x002;
        match instruction {
            Instruction::_00E0 => {
                self.display_state.clear();
            }
            Instruction::_00EE => {
                self.program_counter = self.stack.pop().unwrap() as usize;
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
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                self.registers.write_vx(0x0F, if x > y { 1 } else { 0 });
                self.registers.write_vx(reg_x as usize, x.wrapping_sub(y));
            }
            Instruction::_8xy6(reg_x) => {
                let x = self.registers.read_vx(reg_x as usize);
                self.registers
                    .write_vx(0x0F, if get_bit_from_byte(7, &x) { 1 } else { 0 });
                self.registers.write_vx(reg_x as usize, x >> 1);
            }
            Instruction::_8xy7(reg_x, reg_y) => {
                let x = self.registers.read_vx(reg_x as usize);
                let y = self.registers.read_vx(reg_y as usize);
                self.registers.write_vx(0x0F, if y > x { 1 } else { 0 });
                self.registers.write_vx(reg_x as usize, y.wrapping_sub(x));
            }
            Instruction::_8xyE(reg_x) => {
                let x = self.registers.read_vx(reg_x as usize);
                self.registers
                    .write_vx(0x0F, if get_bit_from_byte(0, &x) { 1 } else { 0 });
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
                self.program_counter = addr as usize + v0 as usize;
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
            Instruction::_Ex9E(register) => {
                let expected_key = self.registers.read_vx(register as usize);
                if self.is_pressed(expected_key) {
                    self.program_counter += 2;
                }
            }
            Instruction::_ExA1(register) => {
                let expected_key = self.registers.read_vx(register as usize);
                if !self.is_pressed(expected_key) {
                    self.program_counter += 2;
                }
            }
            Instruction::_Fx07(register) => {
                self.registers
                    .write_vx(register as usize, self.registers.read_delay_timer());
            }
            Instruction::_Fx0A(register) => {
                if let Some(key) = self.current_key {
                    self.registers.write_vx(register as usize, key);
                } else {
                    self.program_counter -= 2;
                }
            }
            Instruction::_Fx15(register) => {
                self.registers
                    .set_delay_timer(self.registers.read_vx(register as usize));
            }
            Instruction::_Fx18(register) => {
                self.registers
                    .set_sound_timer(self.registers.read_vx(register as usize));
            }
            Instruction::_Fx1E(register) => {
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

    pub fn tick(&mut self) {
        self.tick += 1;
        if self.tick % 15 == 0 {
            self.decr_timers();
            self.tick = 0;
        }
        let next_instruction: Instruction = self.ram.read_bytes(self.program_counter, 2).into();
        self.run_instruction(next_instruction);
        self.display.draw(&self.display_state.as_bytes());
    }
}

impl Debug for Chip8<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.registers).unwrap();
        write!(f, "{:?}", self.stack).unwrap();
        write!(f, "{:?}", self.ram)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct DisplayMock {}
    impl Chip8Display for DisplayMock {
        fn draw(&mut self, _bytes: &[u8]) {}
    }

    #[test]
    fn clear() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        // load first letter from the font
        chip8.load_sprite(0, 0, 5);
        assert_ne!(
            chip8.display_state.as_bytes(),
            &[0; DISPLAY_COLS * DISPLAY_ROWS / 8]
        );

        let instruction = Instruction::_00E0;
        chip8.run_instruction(instruction);

        assert_eq!(
            chip8.display_state.as_bytes(),
            &[0; DISPLAY_COLS * DISPLAY_ROWS / 8]
        );
    }

    #[test]
    fn ret() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.stack.push(0x1234);

        let instruction = Instruction::_00EE;
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x1234);
    }

    #[test]
    fn jump() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        let instruction = Instruction::_1nnn(0x1234);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x1234);
    }

    #[test]
    fn call() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.program_counter = 0x1234;

        let instruction = Instruction::_2nnn(0xAABB);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0xAABB);
        assert_eq!(chip8.stack.pop(), Some(0x1236));
    }

    #[test]
    fn skip_eq_skips() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xAB);

        let instruction = Instruction::_3xkk(0x01, 0xAB);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x04);
    }

    #[test]
    fn skip_eq_does_not_skip() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xAB);

        let instruction = Instruction::_3xkk(0x01, 0xAC);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x02);
    }

    #[test]
    fn skip_ne_skips() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xAB);

        let instruction = Instruction::_4xkk(0x01, 0xAC);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x04);
    }

    #[test]
    fn skip_ne_does_not_skip() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xAB);

        let instruction = Instruction::_4xkk(0x01, 0xAB);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x02);
    }

    #[test]
    fn cmp_eq_skips() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xAB);
        chip8.registers.write_vx(0x02, 0xAB);

        let instruction = Instruction::_5xy0(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x04);
    }

    #[test]
    fn cmp_eq_does_not_skip() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xAB);
        chip8.registers.write_vx(0x02, 0xAC);

        let instruction = Instruction::_5xy0(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x02);
    }

    #[test]
    fn write_register() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        let instruction = Instruction::_6xkk(0x01, 0xAB);
        chip8.run_instruction(instruction);
        let instruction = Instruction::_6xkk(0x02, 0xAC);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0xAB);
        assert_eq!(chip8.registers.read_vx(0x02), 0xAC);
    }

    #[test]
    fn add() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 100);

        let instruction = Instruction::_7xkk(0x01, 114);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 214);
    }

    #[test]
    fn add_overflow() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 255);

        let instruction = Instruction::_7xkk(0x01, 2);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 1);
    }

    #[test]
    fn copy() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0x04);
        chip8.registers.write_vx(0x02, 0xF1);

        let instruction = Instruction::_8xy0(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0xF1);
    }

    #[test]
    fn or() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0b00110010);
        chip8.registers.write_vx(0x02, 0b11100101);

        let instruction = Instruction::_8xy1(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0b11110111);
    }

    #[test]
    fn and() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0b00110010);
        chip8.registers.write_vx(0x02, 0b11100101);

        let instruction = Instruction::_8xy2(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0b00100000);
    }

    #[test]
    fn xor() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0b00110010);
        chip8.registers.write_vx(0x02, 0b11100101);

        let instruction = Instruction::_8xy3(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0b11010111);
    }

    #[test]
    fn sum() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 214);
        chip8.registers.write_vx(0x02, 23);

        let instruction = Instruction::_8xy4(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 237);
        assert_eq!(chip8.registers.read_vx(0x0F), 0);
    }

    #[test]
    fn sum_overflow() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 255);
        chip8.registers.write_vx(0x02, 3);

        let instruction = Instruction::_8xy4(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 2);
        assert_eq!(chip8.registers.read_vx(0x0F), 1);
    }

    #[test]
    fn subtract() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 214);
        chip8.registers.write_vx(0x02, 23);

        let instruction = Instruction::_8xy5(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 191);
        assert_eq!(chip8.registers.read_vx(0x0F), 1);
    }

    #[test]
    fn subtract_underflow() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 214);
        chip8.registers.write_vx(0x02, 216);

        let instruction = Instruction::_8xy5(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 254);
        assert_eq!(chip8.registers.read_vx(0x0F), 0);
    }

    #[test]
    fn shift_right() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0b00110100);

        let instruction = Instruction::_8xy6(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0b00011010);
        assert_eq!(chip8.registers.read_vx(0x0F), 0);
    }

    #[test]
    fn shift_right_least_significant() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0b00110101);

        let instruction = Instruction::_8xy6(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0b00011010);
        assert_eq!(chip8.registers.read_vx(0x0F), 1);
    }

    #[test]
    fn subtract_registers() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 200);
        chip8.registers.write_vx(0x02, 215);

        let instruction = Instruction::_8xy7(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 15);
        assert_eq!(chip8.registers.read_vx(0x0F), 1);
    }

    #[test]
    fn subtract_registers_underflow() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 200);
        chip8.registers.write_vx(0x02, 180);

        let instruction = Instruction::_8xy7(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 236);
        assert_eq!(chip8.registers.read_vx(0x0F), 0);
    }

    #[test]
    fn shift_left() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0b01001101);

        let instruction = Instruction::_8xyE(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0b10011010);
        assert_eq!(chip8.registers.read_vx(0x0F), 0);
    }

    #[test]
    fn shift_left_most_significant() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0b11001101);

        let instruction = Instruction::_8xyE(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x01), 0b10011010);
        assert_eq!(chip8.registers.read_vx(0x0F), 1);
    }

    #[test]
    fn skip_cmp_ne_skips() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0x0F);
        chip8.registers.write_vx(0x02, 0x0E);

        let instruction = Instruction::_9xy0(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 4);
    }

    #[test]
    fn skip_cmp_ne_does_not_skip() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0x0F);
        chip8.registers.write_vx(0x02, 0x0F);

        let instruction = Instruction::_9xy0(0x01, 0x02);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 2);
    }

    #[test]
    fn set_addr() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        let instruction = Instruction::_Annn(0x140F);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_i(), 0x140F);
    }

    #[test]
    fn jump_with_reg() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x00, 0x13);

        let instruction = Instruction::_Bnnn(0x23);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0x13 + 0x23);
    }

    #[test]
    fn draw() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        // draw first letter from font
        let instruction = Instruction::_Dxyn(0, 0, 5);
        chip8.run_instruction(instruction);

        assert_eq!(&chip8.display_state.as_bytes()[0], &FONT_SPRITES[0]);
        assert_eq!(&chip8.display_state.as_bytes()[8], &FONT_SPRITES[1]);
        assert_eq!(&chip8.display_state.as_bytes()[16], &FONT_SPRITES[2]);
        assert_eq!(&chip8.display_state.as_bytes()[24], &FONT_SPRITES[3]);
        assert_eq!(&chip8.display_state.as_bytes()[32], &FONT_SPRITES[4]);

        // draw same sprite -> should flip bits
        let instruction = Instruction::_Dxyn(0, 0, 5);
        chip8.run_instruction(instruction);

        assert_eq!(&chip8.display_state.as_bytes()[0], &0x00);
        assert_eq!(&chip8.display_state.as_bytes()[8], &0x00);
        assert_eq!(&chip8.display_state.as_bytes()[16], &0x00);
        assert_eq!(&chip8.display_state.as_bytes()[24], &0x00);
        assert_eq!(&chip8.display_state.as_bytes()[32], &0x00);
    }

    #[test]
    fn rnd() {
        // we expect this instruction to AND the given value with a random
        // number and store the result in Vx. In this test we use a mask of
        // 255 to AND with the random number so we can assert that the number
        // is indeed at least pseudo-random

        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        let instruction = Instruction::_Cxkk(0x01, 255);
        chip8.run_instruction(instruction);
        let first = chip8.registers.read_vx(0x01);

        let instruction = Instruction::_Cxkk(0x01, 255);
        chip8.run_instruction(instruction);
        let second = chip8.registers.read_vx(0x01);

        assert_ne!(first, second);
    }

    #[test]
    fn skip_if_key_skips() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xA);
        chip8.register_key(0xA);
        let instruction = Instruction::_Ex9E(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 4);
    }

    #[test]
    fn skip_if_key_does_not_skip() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0xA);
        chip8.register_key(0xB);
        let instruction = Instruction::_Ex9E(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 2);
    }

    #[test]
    fn skip_if_not_key_skips() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0x0A);
        chip8.register_key(0xB);
        let instruction = Instruction::_ExA1(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 4);
    }

    #[test]
    fn skip_if_not_key_does_not_skip() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0x0A);
        chip8.register_key(0xA);
        let instruction = Instruction::_ExA1(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 2);
    }

    #[test]
    fn wait_for_key_waits() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.current_key = None;
        let instruction = Instruction::_Fx0A(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 0);
    }

    #[test]
    fn wait_for_key_stores_key() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.current_key = Some(0x0F);
        let instruction = Instruction::_Fx0A(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.program_counter, 2);
        assert_eq!(chip8.registers.read_vx(0x01), 0x0F);
    }

    #[test]
    fn set_delay_timer() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 30);
        let instruction = Instruction::_Fx15(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_delay_timer(), 30);
    }

    #[test]
    fn set_sound_timer() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 30);
        let instruction = Instruction::_Fx18(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_sound_timer(), 30);
    }

    #[test]
    fn add_register_to_addr() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 0x0004);
        chip8.registers.write_i(0x000F);
        let instruction = Instruction::_Fx29(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_i(), 0x0014);
    }

    #[test]
    fn set_font_addr() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        // the F sprite starts at byte 75
        chip8.registers.write_vx(0x01, 0x0F);
        let instruction = Instruction::_Fx29(0x01);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_i(), 75);
    }

    #[test]
    fn binary_coded_decimal() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_vx(0x01, 123);
        let instruction = Instruction::_Fx33(0x01);
        chip8.run_instruction(instruction);

        let expected: [u8; 3] = [1, 2, 3];

        assert_eq!(
            chip8.ram.read_bytes(chip8.registers.read_i() as usize, 3),
            &expected
        );
    }

    #[test]
    fn load_registers_in_ram() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_i(PROGRAM_OFFSET as u16);
        chip8.registers.write_vx(0x00, 0x0A);
        chip8.registers.write_vx(0x01, 0x0F);
        chip8.registers.write_vx(0x02, 0x01);
        chip8.registers.write_vx(0x03, 0x07);

        let instruction = Instruction::_Fx55(0x03);
        chip8.run_instruction(instruction);

        let expected: [u8; 5] = [0x0A, 0x0F, 0x01, 0x07, 0x00]; // last byte unaffected

        assert_eq!(
            chip8.ram.read_bytes(chip8.registers.read_i() as usize, 5),
            &expected
        );
    }

    #[test]
    fn load_ram_to_registers() {
        let mut display = DisplayMock {};
        let mut chip8 = Chip8::new(&mut display);

        chip8.registers.write_i(PROGRAM_OFFSET as u16);
        chip8
            .ram
            .write_bytes(PROGRAM_OFFSET, &[0x04, 0x0F, 0x0A, 0x07]);
        let instruction = Instruction::_Fx65(0x03);
        chip8.run_instruction(instruction);

        assert_eq!(chip8.registers.read_vx(0x00), 0x04);
        assert_eq!(chip8.registers.read_vx(0x01), 0x0F);
        assert_eq!(chip8.registers.read_vx(0x02), 0x0A);
        assert_eq!(chip8.registers.read_vx(0x03), 0x07);
        assert_eq!(chip8.registers.read_vx(0x04), 0x00); // <-- should be unaffacted
    }
}
