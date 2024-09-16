use std::u16;

#[derive(Debug)]
pub struct BufferReader<'a> {
    buffer: &'a [u8],
    read_pos: usize,
}

impl<'a> BufferReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            read_pos: 0,
        }
    }

    pub fn read_bytes(&mut self, length: usize) -> Option<&'a [u8]> {
        let read_bytes = self.buffer.get(self.read_pos..self.read_pos + length);
        self.read_pos += length;
        return read_bytes;
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        let i64_bytes = match self.read_bytes(8) {
            Some(bytes) => bytes,
            None => return None,
        };
        return Some(i64::from_le_bytes(match <[u8; 8]>::try_from(i64_bytes) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        }));
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        let u64_bytes = match self.read_bytes(8) {
            Some(bytes) => bytes,
            None => return None,
        };
        return Some(u64::from_le_bytes(match <[u8; 8]>::try_from(u64_bytes) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        }));
    }
    pub fn read_u32(&mut self) -> Option<u32> {
        let bytes = match self.read_bytes(4) {
            Some(bytes) => bytes,
            None => return None,
        };
        return Some(u32::from_le_bytes(match <[u8; 4]>::try_from(bytes) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        }));
    }
    pub fn read_u16(&mut self) -> Option<u16> {
        let bytes = match self.read_bytes(2) {
            Some(bytes) => bytes,
            None => return None,
        };
        return Some(u16::from_le_bytes(match <[u8; 2]>::try_from(bytes) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        }));
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        let u64_bytes = match self.read_bytes(1) {
            Some(bytes) => bytes,
            None => return None,
        };
        return Some(u8::from_le_bytes(match <[u8; 1]>::try_from(u64_bytes) {
            Ok(bytes) => bytes,
            Err(_) => return None,
        }));
    }

    pub fn get_read_pos(&self) -> usize {
        return self.read_pos;
    }
}
