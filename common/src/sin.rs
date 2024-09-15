use std::{error::Error, fmt::Display};

use sections::SinSection;

use crate::{
    constants::{MAGIC_1, MAGIC_2, MAGIC_3, MAGIC_4},
    memory::buffer_reader::BufferReader,
};

pub mod sections;

#[derive(Debug)]
pub enum SinError {
    InvalidSin,
    InvalidSection,
    InvalidSectionType(u8),
}

impl Display for SinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSin => write!(f, "The data is not valid sin file"),
            Self::InvalidSection => write!(f, "The data contains invalid sections"),
            Self::InvalidSectionType(ty) => {
                write!(f, "The data contains invalid sections type: `{}`", ty)
            }
        }
    }
}

impl Error for SinError {}

pub struct Sin<'a> {
    sections: Vec<SinSection>,
    data: &'a [u8],
}

impl<'a> Sin<'a> {
    pub fn new(sections: Vec<SinSection>, data: &'a [u8]) -> Self {
        Self { data, sections }
    }

    pub fn from_bytes(data: &'a [u8]) -> Result<Self, SinError> {
        let mut sin = BufferReader::new(&data);
        let magic = sin.read_bytes(4).ok_or(SinError::InvalidSin)?;
        if !(magic[0] == MAGIC_1
            && magic[1] == MAGIC_2
            && magic[2] == MAGIC_3
            && magic[3] == MAGIC_4)
        {
            return Err(SinError::InvalidSin);
        }

        let section_count = sin.read_u32().ok_or(SinError::InvalidSin)?;
        let data_len = sin.read_u64().ok_or(SinError::InvalidSin)?;
        let mut sections = Vec::new();
        for _ in 0..section_count {
            sections.push(SinSection::from_reader(&mut sin)?);
        }
        let decoded_data = sin
            .read_bytes(data_len as usize)
            .ok_or(SinError::InvalidSin)?;
        return Ok(Self {
            data: decoded_data,
            sections,
        });
    }

    pub fn data(&self) -> &'a [u8] {
        return self.data;
    }

    pub fn sections(&self) -> &[SinSection] {
        return &self.sections;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&[MAGIC_1, MAGIC_2, MAGIC_3, MAGIC_4]);
        buffer.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.data.len().to_le_bytes());
        self.sections
            .iter()
            .for_each(|section| buffer.extend_from_slice(&section.to_bytes()));
        buffer.extend_from_slice(self.data);

        return buffer;
    }
}
