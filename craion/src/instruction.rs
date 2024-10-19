use std::{error::Error, fmt::Display};

use proc::collect_instruction;

use crate::{
    decoder::{
        argument::{Argument, ArgumentParseError},
        DecoderError,
    },
    executor::ExecutorState,
    memory::MemoryError,
};

mod loados;
mod local;
mod pushu64;
mod retl;

#[derive(Debug)]
pub enum InstructionError {
    ArgumentParseError(ArgumentParseError),
    AccessingMemoryError(MemoryError),
    InvalidUTF8,
    AddressToRegisterError(usize),
    InvalidSubOpCode(u16, u8),
    InvalidSection(u64),
    EmptyRetStack,
    NotProcedureSection,
    SavedNonGeneral,
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentParseError(argument_e) => write!(f, "{}", argument_e),
            Self::AccessingMemoryError(memory_e) => {
                write!(
                    f,
                    "An instruction trying to access memory with error '{}'",
                    memory_e
                )
            }
            Self::InvalidUTF8 => {
                write!(f, "Invalid UTF8")
            }
            Self::InvalidSubOpCode(mainopcode, subopcode) => write!(
                f,
                "Trying to execute invalid sup op code. Main OP Code {}, Sub OP Code: {}",
                mainopcode, subopcode
            ),
            Self::AddressToRegisterError(size) => write!(
                f,
                "Trying to put an address to a register with size that is not 64 bit; register size: {}",
                size * 8
            ),
            Self::InvalidSection(hash) => write!(f, "Trying to access invalid section with hash: {}", hash),
            Self::EmptyRetStack => write!(f, "Executing return insturction on an empty return stack"),
            Self::NotProcedureSection => write!(f, "Trying to call a section thats not a procedure"),
            Self::SavedNonGeneral => write!(f, "Cannot save a register that is not general purpose")
        }
    }
}

impl Error for InstructionError {}

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

#[derive(Debug)]
pub struct Instruction<'a> {
    instruction_executor: fn(
        state: &mut ExecutorState,
        argument: &mut Argument,
        instruction_length: usize,
    ) -> Result<(), InstructionError>,
    state: &'a mut ExecutorState,
    argument: Argument<'a>,
    instruction_length: usize,
    opcode: u16,
}

impl<'a> Instruction<'a> {
    pub fn decode(
        op_code: u16,
        argument: Argument<'a>,
        executor_state: &'a mut ExecutorState,
        instruction_length: usize,
    ) -> Result<Self, DecoderError> {
        collect_instruction!(op_code, argument, executor_state, instruction_length);
    }

    pub fn execute(&mut self) -> Result<(), InstructionError> {
        return (self.instruction_executor)(
            self.state,
            &mut self.argument,
            self.instruction_length,
        );
    }

    pub fn op_code(&self) -> u16 {
        return self.opcode;
    }
}
