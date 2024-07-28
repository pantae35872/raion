use std::{error::Error, fmt::Display};

use crate::executor::registers::{RegisterFile, RegisterFileError};

use self::mov::Mov;

use super::{
    argument::{Argument, ArgumentParseError},
    DecoderError,
};

pub mod mov;

#[derive(Debug)]
pub enum InstructionError {
    ArgumentParseError(ArgumentParseError),
    RegisterFileError(RegisterFileError),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::ArgumentParseError(argument_e) => write!(f, "{}", argument_e),
            InstructionError::RegisterFileError(register_e) => write!(f, "{}", register_e),
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

pub fn decode<'a>(
    op_code: u16,
    register: &'a mut RegisterFile,
    argument: &'a mut Argument<'a>,
) -> Result<impl Instruction + 'a, DecoderError> {
    match op_code {
        16 => return Ok(Mov::new(register, argument)),
        iop_code => return Err(DecoderError::InvalidOpCode(iop_code)),
    }
}

pub trait Instruction {
    fn execute(&mut self) -> Result<(), InstructionError>;
}
