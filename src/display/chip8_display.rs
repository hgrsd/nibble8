pub trait Chip8Display {
    fn draw(&mut self, bytes: &[u8]);
}
