use std::{error::Error, fmt::Display};

use crate::memory::buffer_reader::BufferReader;

#[derive(Debug)]
pub enum SinError {
    InvalidSin,
}

impl Display for SinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSin => write!(f, "The data is not valid sin file"),
        }
    }
}

impl Error for SinError {}

pub struct Sin<'a> {
    text: &'a [u8],
    data: &'a [u8],
}

impl<'a> Sin<'a> {
    pub fn new(text: &'a [u8], data: &'a [u8]) -> Self {
        Self { text, data }
    }

    pub fn from_bytes(data: &'a [u8]) -> Result<Self, SinError> {
        let mut sin = BufferReader::new(&data);
        let magic = sin.read_bytes(4).ok_or(SinError::InvalidSin)?;
        if !(magic[0] == 69 && magic[1] == 69 && magic[2] == 0x69 && magic[3] == 0x69) {
            return Err(SinError::InvalidSin);
        }

        let text_len = sin.read_u64().ok_or(SinError::InvalidSin)?;
        let data_len = sin.read_u64().ok_or(SinError::InvalidSin)?;
        let decoded_text = sin
            .read_bytes(text_len as usize)
            .ok_or(SinError::InvalidSin)?;
        let decoded_data = sin
            .read_bytes(data_len as usize)
            .ok_or(SinError::InvalidSin)?;
        return Ok(Self {
            text: decoded_text,
            data: decoded_data,
        });
    }

    pub fn text(&self) -> &'a [u8] {
        return self.text;
    }

    pub fn data(&self) -> &'a [u8] {
        return self.data;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&[69, 69, 0x69, 0x69]);
        buffer.extend_from_slice(&self.text.len().to_le_bytes());
        buffer.extend_from_slice(&self.data.len().to_le_bytes());
        buffer.extend_from_slice(self.text);
        buffer.extend_from_slice(self.data);

        return buffer;
    }
}
