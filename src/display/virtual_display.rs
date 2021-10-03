pub trait VirtualDisplay {
    fn draw(&mut self, bytes: &[u8]);
}