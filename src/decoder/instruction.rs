use std::{error::Error, fmt::Display};

use cmp::Cmp;
use jmp::Jmp;
use jmz::Jmz;

use crate::{
    executor::registers::{RegisterFile, RegisterFileError},
    memory::Memory,
};

use self::{add::Add, halt::Halt, mov::Mov};

use super::{
    argument::{Argument, ArgumentParseError},
    DecoderError,
};

mod add;
mod cmp;
mod halt;
mod jmp;
mod jmz;
mod mov;

pub const MOV_OPCODE: u16 = 16;
pub const CMP_OPCODE: u16 = 31;
pub const ADD_OPCODE: u16 = 32;
pub const JMP_OPCODE: u16 = 33;
pub const JMZ_OPCODE: u16 = 34;
pub const HALT_OPCODE: u16 = 65535;

#[derive(Debug)]
pub enum InstructionError {
    ArgumentParseError(ArgumentParseError),
    RegisterFileError(RegisterFileError),
    InvalidSubOpCode(u16, u8),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentParseError(argument_e) => write!(f, "{}", argument_e),
            Self::RegisterFileError(register_e) => write!(f, "{}", register_e),
            Self::InvalidSubOpCode(mainopcode, subopcode) => write!(
                f,
                "Trying to execute invalid sup op code. Main OP Code {}, Sub OP Code: {}",
                mainopcode, subopcode
            ),
        }
    }
}

impl Error for InstructionError {}

impl From<RegisterFileError> for InstructionError {
    fn from(value: RegisterFileError) -> Self {
        Self::RegisterFileError(value)
    }
}

impl From<ArgumentParseError> for InstructionError {
    fn from(value: ArgumentParseError) -> Self {
        Self::ArgumentParseError(value)
    }
}

//TODO: Change to enum insted of trait for performance
pub fn decode<'a>(
    op_code: u16,
    register: &'a mut RegisterFile,
    memory: &'a mut Memory,
    argument: Argument<'a>,
    instruction_length: usize,
) -> Result<Box<dyn Instruction + 'a>, DecoderError> {
    match op_code {
        MOV_OPCODE => {
            return Ok(Box::from(Mov::new(
                register,
                memory,
                argument,
                instruction_length,
            )))
        }
        ADD_OPCODE => return Ok(Box::from(Add::new(register, argument, instruction_length))),
        HALT_OPCODE => return Ok(Box::from(Halt::new(register, instruction_length))),
        JMP_OPCODE => return Ok(Box::from(Jmp::new(register, argument))),
        JMZ_OPCODE => return Ok(Box::from(Jmz::new(register, argument, instruction_length))),
        CMP_OPCODE => return Ok(Box::from(Cmp::new(register, argument, instruction_length))),
        iop_code => return Err(DecoderError::InvalidOpCode(iop_code)),
    }
}

pub trait Instruction {
    fn execute(&mut self) -> Result<(), InstructionError>;
    fn op_code(&self) -> u16;
}
