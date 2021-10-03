pub fn get_bit_from_byte(bit: usize, byte: &u8) -> bool {
    let mask = 0b10000000 >> bit;
    let value = (byte & mask) >> (7 - bit);
    value == 1
}
