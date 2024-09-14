use std::{error::Error, fmt::Display};

use common::constants::{
    ADD_OPCODE, CMP_OPCODE, HALT_OPCODE, INC_OPCODE, JACC_OPCODE, JACE_OPCODE, JACN_OPCODE,
    JACZ_OPCODE, JMC_OPCODE, JME_OPCODE, JMN_OPCODE, JMP_OPCODE, JMZ_OPCODE, MOV_OPCODE,
    POP_OPCODE, PUSH_OPCODE, SUB_OPCODE,
};
use proc::collect_instruction;

use crate::{
    executor::registers::{RegisterFile, RegisterFileError},
    memory::{Memory, MemoryError},
};

use super::{
    argument::{Argument, ArgumentParseError},
    DecoderError,
};

mod add;
mod cmp;
mod halt;
mod inc;
mod jacc;
mod jace;
mod jacn;
mod jacz;
mod jmc;
mod jme;
mod jmn;
mod jmp;
mod jmz;
pub mod mov;
mod pop;
mod push;
mod sub;

#[derive(Debug)]
pub enum InstructionError {
    ArgumentParseError(ArgumentParseError),
    RegisterFileError(RegisterFileError),
    AccessingMemoryError(MemoryError),
    InvalidSubOpCode(u16, u8),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentParseError(argument_e) => write!(f, "{}", argument_e),
            Self::RegisterFileError(register_e) => write!(f, "{}", register_e),
            Self::AccessingMemoryError(memory_e) => {
                write!(
                    f,
                    "An instruction trying to access memory with error '{}'",
                    memory_e
                )
            }
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

impl From<MemoryError> for InstructionError {
    fn from(value: MemoryError) -> Self {
        Self::AccessingMemoryError(value)
    }
}

impl From<ArgumentParseError> for InstructionError {
    fn from(value: ArgumentParseError) -> Self {
        Self::ArgumentParseError(value)
    }
}

pub struct InstructionArgument<'a> {
    pub register: &'a mut RegisterFile,
    pub memory: &'a mut Memory,
    pub argument: Argument<'a>,
    pub instruction_length: usize,
}

pub struct Instruction<'a> {
    instruction_executor: fn(&mut InstructionArgument) -> Result<(), InstructionError>,
    instruction_argument: InstructionArgument<'a>,
    opcode: u16,
}

impl<'a> Instruction<'a> {
    pub fn decode(
        op_code: u16,
        register: &'a mut RegisterFile,
        memory: &'a mut Memory,
        argument: Argument<'a>,
        instruction_length: usize,
    ) -> Result<Self, DecoderError> {
        collect_instruction!(op_code, register, memory, argument, instruction_length);
    }

    pub fn execute(&mut self) -> Result<(), InstructionError> {
        return (self.instruction_executor)(&mut self.instruction_argument);
    }

    pub fn op_code(&self) -> u16 {
        return self.opcode;
    }
}
