pub struct ArgumentMemory {
    buffer: Vec<u8>,
    lastest_len: usize,
}

impl ArgumentMemory {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            lastest_len: 0,
        }
    }

    pub fn expand_buffer(&mut self, amount: usize) {
        for _ in 0..amount {
            self.buffer.push(0);
        }
    }

    pub fn set_arguement(&mut self, buffer: &[u8]) {
        if self.buffer.len() < buffer.len() {
            self.expand_buffer(buffer.len() - self.buffer.len());
        }
        self.buffer[0..buffer.len()].copy_from_slice(buffer);
        self.lastest_len = buffer.len();
    }

    pub fn get_argument(&self) -> &[u8] {
        return &self.buffer[0..self.lastest_len];
    }
}
