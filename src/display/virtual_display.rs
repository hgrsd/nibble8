pub trait VirtualDisplay {
    fn load(&mut self, x: usize, y: usize, sprite: &[u8]);
    fn clear(&mut self);
}