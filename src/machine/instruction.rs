#[derive(Debug)]
pub enum Instruction {
    _00E0,
    _00EE,
    _1nnn(u16),
    _2nnn(u16),
    _3xkk(u8, u8),
    _4xkk(u8, u8),
    _5xy0(u8, u8),
    _6xkk(u8, u8),
    _7xkk(u8, u8),
    _8xy0(u8, u8),
    _8xy1(u8, u8),
    _8xy2(u8, u8),
    _8xy3(u8, u8),
    _8xy4(u8, u8),
    _8xy5(u8, u8),
    _8xy6(u8, u8),
    _8xy7(u8, u8),
    _8xyE(u8, u8),
    _9xy0(u8, u8),
    _Annn(u16),
    _Bnnn(u16),
    _Cxkk(u8, u8),
    _Dxyn(u8, u8, u8),
    _Ex9E(u8),
    _ExA1(u8),
    _Fx07(u8),
    _Fx0A(u8),
    _Fx15(u8),
    _Fx18(u8),
    _Fx1E(u8),
    _Fx29(u8),
    _Fx33(u8),
    _Fx55(u8),
    _Fx65(u8),
}

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 2);
        let op_type = bytes[0] >> 4;
        let x: u8 = bytes[0] & 0x0F;

        let y: u8 = bytes[1] >> 4;
        let kk: u8 = bytes[1];
        let n: u8 = bytes[1] & 0x0F;

        let combined: u16 = (bytes[0] as u16) << 8 | bytes[1];
        let nnn: u16 = combined & 0x0FFF;

        match (op_type, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::_00E0,
            (0x0, 0x0, 0xE, 0xE) => Instruction::_00EE,
            (0x1, _, _, _) => Instruction::_1nnn(nnn),
            (0x2, _, _, _) => Instruction::_2nnn(nnn),
            (0x3, _, _, _) => Instruction::_3xkk(x, kk),
            (0x4, _, _, _) => Instruction::_4xkk(x, kk),
            (0x5, _, _, _) => Instruction::_5xy0(x, y),
            (0x6, _, _, _) => Instruction::_6xkk(x, kk),
            (0x7, _, _, _) => Instruction::_7xkk(x, kk),
            (0x8, _, _, 0x0) => Instruction::_8xy0(x, y),
            (0x8, _, _, 0x1) => Instruction::_8xy1(x, y),
            (0x8, _, _, 0x2) => Instruction::_8xy2(x, y),
            (0x8, _, _, 0x3) => Instruction::_8xy3(x, y),
            (0x8, _, _, 0x4) => Instruction::_8xy4(x, y),
            (0x8, _, _, 0x5) => Instruction::_8xy5(x, y),
            (0x8, _, _, 0x6) => Instruction::_8xy6(x, y),
            (0x8, _, _, 0x7) => Instruction::_8xy7(x, y),
            (0x8, _, _, 0xE) => Instruction::_8xyE(x, y),
            (0x9, _, _, 0x0) => Instruction::_9xy0(x, y),
            (0xA, _, _, _) => Instruction::_Annn(nnn),
            (0xB, _, _, _) => Instruction::_Bnnn(nnn),
            (0xC, _, _, _) => Instruction::_Cxkk(x, kk),
            (0xD, _, _, _) => Instruction::_Dxyn(x, y, n),
            (0xE, _, 0x9, 0xE) => Instruction::_Ex9E(x),
            (0xE, _, 0xA, 0x1) => Instruction::_ExA1(x),
            (0xF, _, 0x0, 0x7) => Instruction::_Fx07(x),
            (0xF, _, 0x0, 0xA) => Instruction::_Fx0A(x),
            (0xF, _, 0x1, 0x5) => Instruction::_Fx15(x),
            (0xF, _, 0x1, 0x8) => Instruction::_Fx18(x),
            (0xF, _, 0x1, 0xE) => Instruction::_Fx1E(x),
            (0xF, _, 0x2, 0x9) => Instruction::_Fx29(x),
            (0xF, _, 0x3, 0x3) => Instruction::_Fx33(x),
            (0xF, _, 0x5, 0x5) => Instruction::_Fx55(x),
            (0xF, _, 0x6, 0x5) => Instruction::_Fx65(x),
            _ => unreachable!(),
        }
    }
}
