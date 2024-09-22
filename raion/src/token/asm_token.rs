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
pub enum ASMToken {
    Instruction(InstructionType),
    Label(String),
    Register(RegisterType),
    Interger(u64),
    Identifier(String),
    String(String),
    Plus,
    Minus,
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
            Self::Identifier(label) => write!(f, "Identifier token with value: {}", label),
            Self::String(string) => write!(f, "String token with value: {}", string),
            Self::Plus => write!(f, "Plus token"),
            Self::LBracket => write!(f, "Left Bracket token"),
            Self::RBracket => write!(f, "Right Bracket token"),
            Self::LCurly => write!(f, "Left Curly token"),
            Self::RCurly => write!(f, "Right Curly token"),
            Self::Arrow => write!(f, "Arrow token"),
            Self::Comma => write!(f, "Comma token"),
            Self::Minus => write!(f, "Minus token"),
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
