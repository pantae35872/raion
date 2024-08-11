use std::{fmt::Display, str::FromStr};

pub const MOV_REG2REG: u8 = 1;
pub const MOV_REG2MEM: u8 = 2;
pub const MOV_NUM2REG: u8 = 3;
pub const MOV_ADD2SP: u8 = 4;
pub const MOV_REG2SP: u8 = 5;
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Add,
    Cmp,
    Halt,
    Inc,
    Sub,
    Push,
    Pop,
    Mov,
    Jmp,
    Jmn,
    Jme,
    Jmz,
    Jmc,
    Jacc,
    Jace,
    Jacn,
    Jacz,
}

#[derive(Debug, Clone, PartialEq)]
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
}

//Memory releate instructions
impl InstructionType {
    pub fn opcode(&self) -> u16 {
        match self {
            Self::Mov => return 16,
            Self::Push => return 17,
            Self::Pop => return 18,
            Self::Inc => return 30,
            Self::Cmp => return 31,
            Self::Add => return 32,
            Self::Sub => return 33,
            Self::Jmp => return 64,
            Self::Jmz => return 65,
            Self::Jmn => return 66,
            Self::Jacn => return 67,
            Self::Jacz => return 68,
            Self::Jacc => return 69,
            Self::Jace => return 70,
            Self::Jme => return 71,
            Self::Jmc => return 72,
            Self::Halt => return 65535,
        }
    }
}

impl RegisterType {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::A8 => return 1,
            Self::A16 => return 2,
            Self::A32 => return 3,
            Self::A64 => return 4,
            Self::B8 => return 5,
            Self::B16 => return 6,
            Self::B32 => return 7,
            Self::B64 => return 8,
            Self::C8 => return 9,
            Self::C16 => return 10,
            Self::C32 => return 11,
            Self::C64 => return 12,
            Self::D8 => return 13,
            Self::D16 => return 14,
            Self::D32 => return 15,
            Self::D64 => return 16,
            Self::SP => return 254,
            Self::IP => return 255,
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

impl Display for RegisterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A8 => write!(f, "a8"),
            Self::A16 => write!(f, "a16"),
            Self::A32 => write!(f, "a32"),
            Self::A64 => write!(f, "a64"),
            Self::B8 => write!(f, "b8"),
            Self::B16 => write!(f, "b16"),
            Self::B32 => write!(f, "b32"),
            Self::B64 => write!(f, "b64"),
            Self::C8 => write!(f, "c8"),
            Self::C16 => write!(f, "c16"),
            Self::C32 => write!(f, "c32"),
            Self::C64 => write!(f, "c64"),
            Self::D8 => write!(f, "d8"),
            Self::D16 => write!(f, "d16"),
            Self::D32 => write!(f, "d32"),
            Self::D64 => write!(f, "d64"),
            Self::SP => write!(f, "stack pointer"),
            Self::IP => write!(f, "instruction pointer"),
        }
    }
}

impl FromStr for InstructionType {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mov" => return Ok(Self::Mov),
            "add" => return Ok(Self::Add),
            "sub" => return Ok(Self::Sub),
            "inc" => return Ok(Self::Inc),
            "push" => return Ok(Self::Push),
            "pop" => return Ok(Self::Pop),
            "cmp" => return Ok(Self::Cmp),
            "jmp" => return Ok(Self::Jmp),
            "jmn" => return Ok(Self::Jmn),
            "jme" => return Ok(Self::Jme),
            "jmz" => return Ok(Self::Jmz),
            "jmc" => return Ok(Self::Jmz),
            "jacc" => return Ok(Self::Jacc),
            "jace" => return Ok(Self::Jace),
            "jacn" => return Ok(Self::Jacn),
            "jacz" => return Ok(Self::Jacz),
            "halt" => return Ok(Self::Halt),
            _ => return Err(FailToParseFromString),
        }
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Cmp => write!(f, "cmp"),
            Self::Jacz => write!(f, "jacz"),
            Self::Inc => write!(f, "inc"),
            Self::Pop => write!(f, "pop"),
            Self::Push => write!(f, "push"),
            Self::Mov => write!(f, "mov"),
            Self::Jmp => write!(f, "jmp"),
            Self::Jmn => write!(f, "jmn"),
            Self::Jme => write!(f, "jme"),
            Self::Jmz => write!(f, "jmz"),
            Self::Jmc => write!(f, "jmc"),
            Self::Jacc => write!(f, "jacc"),
            Self::Jace => write!(f, "jace"),
            Self::Jacn => write!(f, "jacn"),
            Self::Halt => write!(f, "halt"),
            Self::Sub => write!(f, "sub"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASMToken {
    Instruction(InstructionType),
    Label(String),
    Register(RegisterType),
    Number(u64),
    ToLabel(String),
    Comma,
    NewLine,
}

impl Display for ASMToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instruction(instruction) => {
                write!(f, "Instruction token with value: {}", instruction)
            }
            Self::Label(label) => {
                write!(f, "Label token with value: {}", label)
            }
            Self::Register(register) => {
                write!(f, "Register token with value: {}", register)
            }
            Self::Number(number) => {
                write!(f, "Number token with value: {}", number)
            }
            Self::ToLabel(label) => write!(f, "ToLabel token with value: {}", label),
            Self::Comma => write!(f, "Comma token"),
            Self::NewLine => write!(f, "New line token"),
        }
    }
}
