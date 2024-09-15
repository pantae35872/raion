use crate::memory::buffer_reader::BufferReader;

use super::SinError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionType {
    Function,
    Constant,
}

pub struct SinSection {
    section_type: SectionType,
    hash: u64,
    start: u64,
    end: u64,
}

impl SectionType {
    pub fn from_byte(data: u8) -> Result<Self, SinError> {
        match data {
            1 => return Ok(Self::Function),
            2 => return Ok(Self::Constant),
            ty => return Err(SinError::InvalidSectionType(ty)),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Function => return 1,
            Self::Constant => return 2,
        }
    }
}

impl SinSection {
    pub fn new(section_type: SectionType, hash: u64, start: u64, end: u64) -> Self {
        Self {
            section_type,
            hash,
            start,
            end,
        }
    }

    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let decoded_ty = SectionType::from_byte(reader.read_u8().ok_or(SinError::InvalidSin)?)?;
        let decoded_hash = reader.read_u64().ok_or(SinError::InvalidSin)?;
        let decoded_start = reader.read_u64().ok_or(SinError::InvalidSin)?;
        let decoded_end = reader.read_u64().ok_or(SinError::InvalidSin)?;

        if decoded_start > decoded_end {
            return Err(SinError::InvalidSin);
        }

        return Ok(Self {
            section_type: decoded_ty,
            hash: decoded_hash,
            start: decoded_start,
            end: decoded_end,
        });
    }

    pub fn hash(&self) -> u64 {
        return self.hash;
    }

    pub fn section_type(&self) -> SectionType {
        return self.section_type;
    }

    pub fn start(&self) -> u64 {
        return self.start;
    }

    pub fn end(&self) -> u64 {
        return self.end;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.section_type.to_byte());
        buffer.extend_from_slice(&self.hash.to_le_bytes());
        buffer.extend_from_slice(&self.start.to_le_bytes());
        buffer.extend_from_slice(&self.end.to_le_bytes());
        return buffer;
    }
}
