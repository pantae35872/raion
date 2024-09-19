use std::{fmt::Display, str::FromStr};

use super::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    U64,
    U32,
    U16,
    U8,
    I8,
    I16,
    I32,
    I64,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Import,
    Module,
    Procedure,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RinToken {
    Identifier(String),
    String(String),
    Type(PrimitiveType),
    Keyword(Keyword),
    Operator(Operator),
    Interger(u64),
    Dot,
    Comma,
    Equals,
    Semicolon,
    LCurly,
    RCurly,
    LRoundBracket,
    RRoundBracket,
    Colon,
    Arrow,
    NewLine,
}

impl Token for RinToken {
    fn is_newline(&self) -> bool {
        matches!(self, RinToken::NewLine)
    }

    fn from_u64(num: u64) -> Self {
        Self::Interger(num)
    }

    fn from_string(string: String) -> Self {
        Self::String(string)
    }
}

impl Display for RinToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "Identifier token with value `{ident}`"),
            Self::Interger(number) => write!(f, "Interger token with value `{number}`"),
            Self::Type(typ) => write!(f, "Type token with value `{typ}`"),
            Self::String(string) => write!(f, "String token with value `{string}`"),
            Self::Keyword(keyword) => write!(f, "Keyword token with value `{keyword}`"),
            Self::Operator(operator) => write!(f, "Operator token with valie `{operator}`"),
            Self::RRoundBracket => write!(f, "Right Round bracket token"),
            Self::LRoundBracket => write!(f, "Left Round bracket token"),
            Self::LCurly => write!(f, "Left Curly token"),
            Self::RCurly => write!(f, "Right Curly token"),
            Self::Semicolon => write!(f, "Semicolon token"),
            Self::Dot => write!(f, "Dot token"),
            Self::Equals => write!(f, "Equals token"),
            Self::Comma => write!(f, "Comma token"),
            Self::Colon => write!(f, "Colon token"),
            Self::Arrow => write!(f, "Arrow token"),
            Self::NewLine => write!(f, "new line token"),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Multiply => write!(f, "multiply"),
            Self::Divide => write!(f, "divide"),
            Self::Subtract => write!(f, "subtract"),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Import => write!(f, "import"),
            Self::Return => write!(f, "return"),
            Self::Module => write!(f, "module"),
            Self::Procedure => write!(f, "procedure"),
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "boolean"),
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

pub struct InvalidType;

impl FromStr for PrimitiveType {
    type Err = InvalidType;

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
            "bool" => Ok(Self::Bool),
            _ => Err(InvalidType),
        }
    }
}

pub struct InvalidKeyword;

impl FromStr for Keyword {
    type Err = InvalidKeyword;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "import" => Ok(Self::Import),
            "module" => Ok(Self::Module),
            "return" => Ok(Self::Return),
            "proc" => Ok(Self::Procedure),
            _ => Err(InvalidKeyword),
        }
    }
}

#[derive(Debug)]
pub struct InvalidOperator;

impl FromStr for Operator {
    type Err = InvalidOperator;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(InvalidOperator),
        }
    }
}
