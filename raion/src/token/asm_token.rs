use std::{fmt::Display, str::FromStr};

use common::{
    constants::{
        ADD_OPCODE, ARG_OPCODE, CALL_OPCODE, CMP_OPCODE, DIV_OPCODE, ENTER_OPCODE, EXIT_OPCODE,
        HALT_OPCODE, INC_OPCODE, JACC_OPCODE, JACE_OPCODE, JACN_OPCODE, JACZ_OPCODE, JMC_OPCODE,
        JME_OPCODE, JMN_OPCODE, JMP_OPCODE, JMZ_OPCODE, LARG_OPCODE, LEAVE_OPCODE, MOV_OPCODE,
        MUL_OPCODE, OUTC_OPCODE, POP_OPCODE, PUSH_OPCODE, RESTR_OPCODE, RET_OPCODE, SAVR_OPCODE,
        SUB_OPCODE,
    },
    register::RegisterType,
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
    Leave,
    Enter,
    Arg,
    LArg,
    Savr,
    Restr,
    Exit,
    Mul,
    Div,
}

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
            Self::Enter => return ENTER_OPCODE,
            Self::Leave => return LEAVE_OPCODE,
            Self::Arg => return ARG_OPCODE,
            Self::LArg => return LARG_OPCODE,
            Self::Savr => return SAVR_OPCODE,
            Self::Restr => return RESTR_OPCODE,
            Self::Exit => return EXIT_OPCODE,
            Self::Mul => return MUL_OPCODE,
            Self::Div => return DIV_OPCODE,
        }
    }
}

pub struct FailToParseFromString;

impl FromStr for InstructionType {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "mov" => Ok(Self::Mov),
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "inc" => Ok(Self::Inc),
            "push" => Ok(Self::Push),
            "pop" => Ok(Self::Pop),
            "cmp" => Ok(Self::Cmp),
            "jmp" => Ok(Self::Jmp),
            "jmn" => Ok(Self::Jmn),
            "jme" => Ok(Self::Jme),
            "jmz" => Ok(Self::Jmz),
            "jmc" => Ok(Self::Jmz),
            "jacc" => Ok(Self::Jacc),
            "jace" => Ok(Self::Jace),
            "jacn" => Ok(Self::Jacn),
            "jacz" => Ok(Self::Jacz),
            "call" => Ok(Self::Call),
            "ret" => Ok(Self::Ret),
            "outc" => Ok(Self::Outc),
            "halt" => Ok(Self::Halt),
            "enter" => Ok(Self::Enter),
            "leave" => Ok(Self::Leave),
            "arg" => Ok(Self::Arg),
            "larg" => Ok(Self::LArg),
            "savr" => Ok(Self::Savr),
            "restr" => Ok(Self::Restr),
            "exit" => Ok(Self::Exit),
            "mul" => Ok(Self::Mul),
            "div" => Ok(Self::Div),
            _ => Err(FailToParseFromString),
        };
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
            Self::Enter => write!(f, "enter"),
            Self::Leave => write!(f, "leave"),
            Self::Arg => write!(f, "arg"),
            Self::LArg => write!(f, "larg"),
            Self::Savr => write!(f, "savr"),
            Self::Restr => write!(f, "restr"),
            Self::Exit => write!(f, "exit"),
            Self::Mul => write!(f, "mul"),
            Self::Div => write!(f, "div"),
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
    Field,
    Struct,
    Interface,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASMToken {
    Instruction(InstructionType),
    Label(String),
    Register(RegisterType),
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

impl ASMToken {
    pub fn is_register_and_general(&self) -> bool {
        match self {
            Self::Register(reg) => match reg {
                RegisterType::Sp | RegisterType::Ip => return false,
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
            Self::HashName(name) => {
                write!(f, "Hash name token with value: {}", name)
            }
            Self::Identifier(name) => {
                write!(f, "Identifier token with value: {}", name)
            }
            Self::Attribute(attribute) => {
                write!(f, "Attribute token with value: {attribute}")
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

impl FromStr for ASMKeyword {
    type Err = FailToParseFromString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "proc" => Ok(Self::Proc),
            "field" => Ok(Self::Field),
            "struct" => Ok(Self::Struct),
            "interface" => Ok(Self::Interface),
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
