use crate::display::virtual_display::VirtualDisplay;

const GRID_SIZE: usize = 64 * 32; // the original Chip-8 implementation worked with a 64x32 pixel display

pub struct TerminalDisplay {
    buf: [u8; GRID_SIZE],
}

impl TerminalDisplay {
    pub fn new() -> TerminalDisplay {
        TerminalDisplay {
            buf: [0x00; GRID_SIZE],
        }
    }
}

impl VirtualDisplay for TerminalDisplay {
    fn load(&mut self, x: usize, y: usize, sprite: &[u8]) {
        unimplemented!();
    }

    fn clear(&mut self) {
        self.buf = [0x00; GRID_SIZE];
    }
}