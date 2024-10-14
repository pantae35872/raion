use std::{error::Error, fmt::Display};

use sections::SinSection;

use crate::{
    constants::{MAGIC_1, MAGIC_2, MAGIC_3, MAGIC_4, MAJOR, MINOR, PATCH},
    memory::buffer_reader::BufferReader,
};

pub mod sections;

#[derive(Debug)]
pub enum SinError {
    InvalidSin,
    InvalidSection,
    InvalidSectionType(u8),
    UnknownAttribute(u8),
    IncompatibleSin {
        current_version: Version,
        file_version: Version,
    },
}

impl Display for SinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSin => write!(f, "The data is not valid sin file"),
            Self::InvalidSection => write!(f, "The data contains invalid sections"),
            Self::InvalidSectionType(ty) => {
                write!(f, "The data contains invalid sections type: `{}`", ty)
            }
            Self::UnknownAttribute(ty) => {
                write!(f, "The data contains unknown type: `{}`", ty)
            }
            Self::IncompatibleSin {
                current_version,
                file_version,
            } => {
                write!(f, "The provided data version is not compatible with the current version, current: `{current_version}`, provided version: `{file_version}`")
            }
        }
    }
}

impl Error for SinError {}

#[derive(Debug, Clone, Copy)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

#[derive(Debug)]
pub struct Sin<'a> {
    sections: Vec<SinSection>,
    data: &'a [u8],
    version: Version,
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: MAJOR,
            minor: MINOR,
            patch: PATCH,
        }
    }

    pub fn compatible(&self, version: Version) -> bool {
        self.major != version.major
    }

    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let major = reader.read_u8().ok_or(SinError::InvalidSin)?;
        let minor = reader.read_u8().ok_or(SinError::InvalidSin)?;
        let patch = reader.read_u8().ok_or(SinError::InvalidSin)?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.major);
        buffer.push(self.minor);
        buffer.push(self.patch);
        buffer
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl<'a> Sin<'a> {
    pub fn new(sections: Vec<SinSection>, data: &'a [u8]) -> Self {
        Self {
            data,
            sections,
            version: Version::new(),
        }
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
        let version = Version::from_reader(&mut sin)?;
        let current_version = Version::new();
        if !current_version.compatible(version) {
            return Err(SinError::IncompatibleSin {
                current_version,
                file_version: version,
            });
        }

        let data_len = sin.read_u64().ok_or(SinError::InvalidSin)?;
        let mut sections = Vec::new();
        for _ in 0..sin.read_u32().ok_or(SinError::InvalidSin)? {
            sections.push(SinSection::from_reader(&mut sin)?);
        }
        let decoded_data = sin
            .read_bytes(data_len as usize)
            .ok_or(SinError::InvalidSin)?;
        return Ok(Self {
            data: decoded_data,
            sections,
            version,
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
        buffer.extend_from_slice(&self.version.to_bytes());
        buffer.extend_from_slice(&self.data.len().to_le_bytes());
        buffer.extend_from_slice(&(self.sections.len() as u32).to_le_bytes());
        self.sections
            .iter()
            .for_each(|section| buffer.extend_from_slice(&section.to_bytes()));
        buffer.extend_from_slice(self.data);

        return buffer;
    }
}
