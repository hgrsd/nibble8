pub fn get_bit_from_byte(bit: usize, byte: &u8) -> bool {
    let mask = 0b10000000 >> bit;
    let value = (byte & mask) >> (7 - bit);
    value == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let byte = 0b11011100;
        assert_eq!(get_bit_from_byte(0, &byte), true);
        assert_eq!(get_bit_from_byte(1, &byte), true);
        assert_eq!(get_bit_from_byte(2, &byte), false);
        assert_eq!(get_bit_from_byte(3, &byte), true);
        assert_eq!(get_bit_from_byte(4, &byte), true);
        assert_eq!(get_bit_from_byte(5, &byte), true);
        assert_eq!(get_bit_from_byte(6, &byte), false);
        assert_eq!(get_bit_from_byte(7, &byte), false);
    }
}