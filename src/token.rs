use std::str::FromStr;

#[derive(Debug)]
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

#[derive(Debug)]
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
            _ => return Err(FailToParseFromString),
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
            _ => return Err(FailToParseFromString),
        }
    }
}

#[derive(Debug)]
pub enum ASMToken {
    Instruction(InstructionType),
    Label(String),
    Register(RegisterType),
    Number(u64),
    Comma,
    NewLine,
}
