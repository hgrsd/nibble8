use crate::bit_utils::get_bit_from_byte;

pub struct DisplayState {
    raw: Vec<u8>,
    cols: usize,
    rows: usize,
}

impl DisplayState {
    pub fn new(cols: usize, rows: usize) -> Self {
        let raw: Vec<u8> = vec![0x00; cols * rows / 8];
        DisplayState { cols, rows, raw }
    }

    pub fn clear(&mut self) {
        self.raw = vec![0x00; self.cols * self.rows / 8];
    }

    fn identify(&self, x: usize, y: usize) -> (usize, usize) {
        let bit_idx = y * self.rows + x;
        let byte_idx = bit_idx / 8;
        let bit_in_byte = bit_idx % 8;
        (byte_idx, bit_in_byte)
    }

    pub fn is_on(&mut self, x: usize, y: usize) -> bool {
        let (byte_idx, bit_idx) = self.identify(x, y);
        get_bit_from_byte(bit_idx, &self.raw[byte_idx])
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        let (byte_idx, bit_idx) = self.identify(x, y);
        let mask = 0b10000000 >> bit_idx;
        self.raw[byte_idx] ^= mask;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = DisplayState::new(8, 1);
        s.flip(4, 0);
        assert_eq!(s.as_bytes()[0], 0b0001000);
    }
}
