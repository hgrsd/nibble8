pub struct Input {
    pub current_key: Option<u8>,
}

impl Input {
    pub fn new() -> Self {
        Input { current_key: None }
    }

    pub fn clear(&mut self) {
        self.current_key = None;
    }

    pub fn register(&mut self, key: u8) {
        self.current_key = Some(key);
    }
}
