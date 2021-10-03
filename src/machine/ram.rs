use std::fmt::{Debug, Formatter};

pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn initialise() -> Ram {
        Ram {
            memory: [0x00; 4096],
        }
    }

    pub fn read_bytes(&self, offset: usize, bytes: usize) -> &[u8] {
        &self.memory[offset..offset + bytes]
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        self.memory[offset..offset + bytes.len()].copy_from_slice(bytes)
    }
}

impl Debug for Ram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mem: ").unwrap();
        for (offset, byte) in self.memory.iter().enumerate() {
            if offset % 32 == 0 {
                writeln!(f, "").unwrap();
            }
            write!(f, "{:#04X} ", byte).unwrap();
        }
        Ok(())
    }
}
