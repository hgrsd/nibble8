use std::fmt::{Debug, Formatter};

pub struct Stack {
    pointer: usize,
    stack: [u16; 16],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            pointer: 0,
            stack: [0x0000; 16],
        }
    }

    pub fn pop(&mut self) -> u16 {
        let bytes = self.stack[self.pointer];
        if self.pointer > 0 {
            self.pointer -= 1;
        }
        return bytes;
    }

    pub fn push(&mut self, bytes: u16) {
        self.pointer += 1;
        self.stack[self.pointer] = bytes;
    }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Stack:").unwrap();
        for (i, entry) in self.stack.iter().enumerate() {
            writeln!(f, "{:#X}: {:#06X}", i, entry).unwrap();
        }
        Ok(())
    }
}