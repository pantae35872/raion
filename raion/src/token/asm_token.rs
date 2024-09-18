use std::{fmt::Display, str::FromStr};

use common::constants::{
    ADD_OPCODE, CALL_OPCODE, CMP_OPCODE, HALT_OPCODE, INC_OPCODE, JACC_OPCODE, JACE_OPCODE,
    JACN_OPCODE, JACZ_OPCODE, JMC_OPCODE, JME_OPCODE, JMN_OPCODE, JMP_OPCODE, JMZ_OPCODE,
    MOV_OPCODE, OUTC_OPCODE, POP_OPCODE, PUSH_OPCODE, RET_OPCODE, SUB_OPCODE,
};

use super::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Add,
    Cmp,
    Halt,
    Outc,
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
    Call,
    Ret,
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
            Self::Mov => return MOV_OPCODE,
            Self::Push => return PUSH_OPCODE,
            Self::Pop => return POP_OPCODE,
            Self::Inc => return INC_OPCODE,
            Self::Cmp => return CMP_OPCODE,
            Self::Add => return ADD_OPCODE,
            Self::Sub => return SUB_OPCODE,
            Self::Jmp => return JMP_OPCODE,
            Self::Jmz => return JMZ_OPCODE,
            Self::Jmn => return JMN_OPCODE,
            Self::Jacn => return JACN_OPCODE,
            Self::Jacz => return JACZ_OPCODE,
            Self::Jacc => return JACC_OPCODE,
            Self::Jace => return JACE_OPCODE,
            Self::Jme => return JME_OPCODE,
            Self::Jmc => return JMC_OPCODE,
            Self::Call => return CALL_OPCODE,
            Self::Ret => return RET_OPCODE,
            Self::Outc => return OUTC_OPCODE,
            Self::Halt => return HALT_OPCODE,
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
            "call" => return Ok(Self::Call),
            "ret" => return Ok(Self::Ret),
            "outc" => return Ok(Self::Outc),
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
            Self::Outc => write!(f, "outc"),
            Self::Halt => write!(f, "halt"),
            Self::Ret => write!(f, "ret"),
            Self::Call => write!(f, "call"),
            Self::Sub => write!(f, "sub"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASMToken {
    Instruction(InstructionType),
    Label(String),
    Register(RegisterType),
    Interger(u64),
    Identifier(String),
    String(String),
    Comma,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    Arrow,
    NewLine,
}

impl ASMToken {
    pub fn is_register_and_general(&self) -> bool {
        match self {
            Self::Register(reg) => match reg {
                RegisterType::SP | RegisterType::IP => return false,
                _ => return true,
            },
            _ => return false,
        }
    }
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
            Self::Interger(number) => {
                write!(f, "Number token with value: {}", number)
            }
            Self::Identifier(label) => write!(f, "ToLabel token with value: {}", label),
            Self::String(string) => write!(f, "String token with value: {}", string),
            Self::LBracket => write!(f, "Left Bracket token"),
            Self::RBracket => write!(f, "Right Bracket token"),
            Self::LCurly => write!(f, "Left Curly token"),
            Self::RCurly => write!(f, "Right Curly token"),
            Self::Arrow => write!(f, "Arrow token"),
            Self::Comma => write!(f, "Comma token"),
            Self::NewLine => write!(f, "New line token"),
        }
    }
}

impl Token for ASMToken {
    fn is_newline(&self) -> bool {
        match self {
            Self::NewLine => return true,
            _ => return false,
        }
    }

    fn from_string(string: String) -> Self {
        Self::String(string)
    }

    fn from_u64(num: u64) -> Self {
        Self::Interger(num)
    }
}
