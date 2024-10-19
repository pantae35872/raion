use std::{fmt::Display, str::FromStr};

use common::constants::{LOADOS_OPCODE, LOCAL_OPCODE, PUSHU64_OPCODE, RETL_OPCODE};

use super::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum IntergerType {
    U64,
    U32,
    U16,
    U8,
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Local,
    RetL,
    PushU64,
    LoadOs,
}

impl InstructionType {
    pub fn opcode(&self) -> u16 {
        match self {
            Self::Local => LOCAL_OPCODE,
            Self::RetL => RETL_OPCODE,
            Self::PushU64 => PUSHU64_OPCODE,
            Self::LoadOs => LOADOS_OPCODE,
        }
    }
}

pub struct FailToParseFromString;

impl FromStr for InstructionType {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "local" => Ok(Self::Local),
            "retl" => Ok(Self::RetL),
            "pushu64" => Ok(Self::PushU64),
            "loados" => Ok(Self::LoadOs),
            _ => Err(FailToParseFromString),
        };
    }
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::RetL => write!(f, "retl"),
            Self::PushU64 => write!(f, "pushu64"),
            Self::LoadOs => write!(f, "loados"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeToken {
    Public,
    Private,
    Implemented,
    Contain,
    Static,
    Accept,
    Return,
    Overwrite,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ASMKeyword {
    Proc,
    VProc,
    Field,
    Struct,
    Interface,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASMToken {
    Instruction(InstructionType),
    Label(String),
    IntergerType(IntergerType),
    Keyword(ASMKeyword),
    Interger(u64),
    Identifier(String),
    HashName(String),
    String(String),
    Attribute(AttributeToken),
    Plus,
    Minus,
    Comma,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    LRoundBracket,
    RRoundBracket,
    Arrow,
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
            Self::Interger(number) => {
                write!(f, "Number token with value: {}", number)
            }
            Self::HashName(name) => {
                write!(f, "Hash name token with value: {}", name)
            }
            Self::Identifier(name) => {
                write!(f, "Identifier token with value: {}", name)
            }
            Self::Attribute(attribute) => {
                write!(f, "Attribute token with value: {attribute}")
            }
            Self::IntergerType(interger_type) => {
                write!(f, "IntergerType token with valie: {interger_type}")
            }
            Self::Keyword(keyword) => write!(f, "Keyword token with value: {keyword}"),
            Self::String(string) => write!(f, "String token with value: {}", string),
            Self::Plus => write!(f, "Plus token"),
            Self::LBracket => write!(f, "Left Bracket token"),
            Self::RBracket => write!(f, "Right Bracket token"),
            Self::LCurly => write!(f, "Left Curly token"),
            Self::RCurly => write!(f, "Right Curly token"),
            Self::LRoundBracket => write!(f, "Left Round Bracket token"),
            Self::RRoundBracket => write!(f, "Right Round Bracket token"),
            Self::Arrow => write!(f, "Arrow token"),
            Self::Comma => write!(f, "Comma token"),
            Self::Minus => write!(f, "Minus token"),
            Self::NewLine => write!(f, "New line token"),
        }
    }
}

impl Display for IntergerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U64 => write!(f, "unsigned 64 bit interger"),
            Self::U32 => write!(f, "unsigned 32 bit interger"),
            Self::U16 => write!(f, "unsigned 16 bit interger"),
            Self::U8 => write!(f, "unsigned 8 bit interger"),
            Self::I8 => write!(f, "signed 8 bit interger"),
            Self::I16 => write!(f, "signed 16 bit interger"),
            Self::I32 => write!(f, "signed 32 bit interger"),
            Self::I64 => write!(f, "signed 64 bit interger"),
        }
    }
}

impl FromStr for IntergerType {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "u64" => Ok(Self::U64),
            "u32" => Ok(Self::U32),
            "u16" => Ok(Self::U16),
            "u8" => Ok(Self::U8),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            _ => Err(FailToParseFromString),
        }
    }
}

impl FromStr for ASMKeyword {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "proc" => Ok(Self::Proc),
            "field" => Ok(Self::Field),
            "struct" => Ok(Self::Struct),
            "interface" => Ok(Self::Interface),
            "vproc" => Ok(Self::VProc),
            _ => Err(FailToParseFromString),
        }
    }
}

impl Display for ASMKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Proc => write!(f, "proc"),
            Self::Field => write!(f, "field"),
            Self::Struct => write!(f, "struct"),
            Self::Interface => write!(f, "interface"),
            Self::VProc => write!(f, "vproc"),
        }
    }
}

impl FromStr for AttributeToken {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Public" => Ok(Self::Public),
            "Private" => Ok(Self::Private),
            "Static" => Ok(Self::Static),
            "Accept" => Ok(Self::Accept),
            "Implemented" => Ok(Self::Implemented),
            "Contain" => Ok(Self::Contain),
            "Return" => Ok(Self::Return),
            "Overwrite" => Ok(Self::Overwrite),
            _ => Err(FailToParseFromString),
        }
    }
}

impl Display for AttributeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "Public"),
            Self::Static => write!(f, "Static"),
            Self::Private => write!(f, "Private"),
            Self::Accept => write!(f, "Accept"),
            Self::Implemented => write!(f, "Implemented"),
            Self::Contain => write!(f, "Contain"),
            Self::Return => write!(f, "Return"),
            Self::Overwrite => write!(f, "Overwrite"),
        }
    }
}

impl Token for ASMToken {
    fn from_string(string: String) -> Self {
        Self::String(string)
    }

    fn from_u64(num: u64) -> Self {
        Self::Interger(num)
    }
}
