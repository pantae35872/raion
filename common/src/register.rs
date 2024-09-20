use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterType {
    A8,
    A16,
    A32,
    A64,
    B8,
    B16,
    B32,
    B64,
    C8,
    C16,
    C32,
    C64,
    D8,
    D16,
    D32,
    D64,
    IP,
    SP,
    FLAGS,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterSizes {
    SizeU8,
    SizeU16,
    SizeU32,
    SizeU64,
}

impl RegisterSizes {
    pub fn byte(&self) -> usize {
        match self {
            RegisterSizes::SizeU8 => return 1,
            RegisterSizes::SizeU16 => return 2,
            RegisterSizes::SizeU32 => return 4,
            RegisterSizes::SizeU64 => return 8,
        }
    }
}

impl Display for RegisterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterType::A8 => write!(f, "a8"),
            RegisterType::A16 => write!(f, "a16"),
            RegisterType::A32 => write!(f, "a32"),
            RegisterType::A64 => write!(f, "a64"),
            RegisterType::B8 => write!(f, "b8"),
            RegisterType::B16 => write!(f, "b16"),
            RegisterType::B32 => write!(f, "b32"),
            RegisterType::B64 => write!(f, "b64"),
            RegisterType::C8 => write!(f, "c8"),
            RegisterType::C16 => write!(f, "c16"),
            RegisterType::C32 => write!(f, "c32"),
            RegisterType::C64 => write!(f, "c64"),
            RegisterType::D8 => write!(f, "d8"),
            RegisterType::D16 => write!(f, "d16"),
            RegisterType::D32 => write!(f, "d32"),
            RegisterType::D64 => write!(f, "d64"),
            RegisterType::IP => write!(f, "instruction pointer"),
            RegisterType::FLAGS => write!(f, "flags"),
            RegisterType::SP => write!(f, "stack pointer"),
        }
    }
}

#[derive(Debug)]
pub enum RegisterParseError {
    InvalidByteForm(u8),
}

impl Display for RegisterParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidByteForm(byte_form) => write!(
                f,
                "Trying to parse from invalid byte respentation of a register: {}",
                byte_form
            ),
        }
    }
}

impl Error for RegisterParseError {}

impl RegisterType {
    pub fn from_byte(byte_form: u8) -> Result<Self, RegisterParseError> {
        return match byte_form {
            1 => Ok(Self::A8),
            2 => Ok(Self::A16),
            3 => Ok(Self::A32),
            4 => Ok(Self::A64),
            5 => Ok(Self::B8),
            6 => Ok(Self::B16),
            7 => Ok(Self::B32),
            8 => Ok(Self::B64),
            9 => Ok(Self::C8),
            10 => Ok(Self::C16),
            11 => Ok(Self::C32),
            12 => Ok(Self::C64),
            13 => Ok(Self::D8),
            14 => Ok(Self::D16),
            15 => Ok(Self::D32),
            16 => Ok(Self::D64),
            253 => Ok(Self::FLAGS),
            254 => Ok(Self::SP),
            255 => Ok(Self::IP),
            e => Err(RegisterParseError::InvalidByteForm(e)),
        };
    }

    pub fn to_byte(&self) -> u8 {
        return match self {
            Self::A8 => 1,
            Self::A16 => 2,
            Self::A32 => 3,
            Self::A64 => 4,
            Self::B8 => 5,
            Self::B16 => 6,
            Self::B32 => 7,
            Self::B64 => 8,
            Self::C8 => 9,
            Self::C16 => 10,
            Self::C32 => 11,
            Self::C64 => 12,
            Self::D8 => 13,
            Self::D16 => 14,
            Self::D32 => 15,
            Self::D64 => 16,
            Self::FLAGS => 253,
            Self::SP => 254,
            Self::IP => 255,
        };
    }

    pub fn size(&self) -> RegisterSizes {
        match self {
            Self::A8 | Self::B8 | Self::C8 | Self::D8 => return RegisterSizes::SizeU8,
            Self::A16 | Self::B16 | Self::C16 | Self::D16 | Self::FLAGS => {
                return RegisterSizes::SizeU16;
            }
            Self::A32 | Self::B32 | Self::C32 | Self::D32 => return RegisterSizes::SizeU32,
            Self::A64 | Self::B64 | Self::C64 | Self::D64 | Self::IP | Self::SP => {
                return RegisterSizes::SizeU64;
            }
        }
    }
}

pub struct FailToParseFromString;

impl FromStr for RegisterType {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a8" => return Ok(Self::A8),
            "a16" => return Ok(Self::A16),
            "a32" => return Ok(Self::A32),
            "a64" => return Ok(Self::A64),
            "b8" => return Ok(Self::B8),
            "b16" => return Ok(Self::B16),
            "b32" => return Ok(Self::B32),
            "b64" => return Ok(Self::B64),
            "c8" => return Ok(Self::C8),
            "c16" => return Ok(Self::C16),
            "c32" => return Ok(Self::C32),
            "c64" => return Ok(Self::C64),
            "d8" => return Ok(Self::D8),
            "d16" => return Ok(Self::D16),
            "d32" => return Ok(Self::D32),
            "d64" => return Ok(Self::D64),
            "sp" => return Ok(Self::SP),
            "ip" => return Ok(Self::IP),
            _ => return Err(FailToParseFromString),
        }
    }
}
