use std::{error::Error, fmt::Display};

use common::{inline_if, register::RegisterType};
use proc::collect_instruction;

use crate::{
    executor::{
        registers::{RegisterFile, RegisterFileError},
        ExecutorState,
    },
    memory::{Memory, MemoryError},
    ret_stack::RetStack,
    section_manager::{LoadedSection, SectionManager},
};

use super::{
    argument::{Argument, ArgumentParseError},
    DecoderError,
};

macro_rules! parse_and_jump {
    ($args:expr) => {
        let section_hash = $args.argument.parse_u64()?;
        let current_section = $args
            .section_manager
            .get_section_hash(section_hash)
            .ok_or(super::InstructionError::InvalidSection(section_hash))?;
        $args
            .register
            .set_ip(current_section.mem_start() + $args.argument.parse_u16()?.into());
    };
}

mod add;
mod arg;
mod call;
mod cmp;
mod enter;
mod exit;
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
mod larg;
mod leave;
mod mov;
mod outc;
mod pop;
mod push;
mod restr;
mod ret;
mod savr;
mod sub;

#[derive(Debug)]
pub enum InstructionError {
    ArgumentParseError(ArgumentParseError),
    RegisterFileError(RegisterFileError),
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
            Self::RegisterFileError(register_e) => write!(f, "{}", register_e),
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

#[derive(Debug)]
pub struct InstructionArgument<'a> {
    pub register: &'a mut RegisterFile,
    pub memory: &'a mut Memory,
    pub argument: Argument<'a>,
    pub ret_stack: &'a mut RetStack,
    pub section_manager: &'a mut SectionManager,
    pub instruction_length: usize,
    pub executor_state: &'a mut ExecutorState,
}

#[derive(Debug)]
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
        ret_stack: &'a mut RetStack,
        section_manager: &'a mut SectionManager,
        executor_state: &'a mut ExecutorState,
        instruction_length: usize,
    ) -> Result<Self, DecoderError> {
        collect_instruction!(
            op_code,
            register,
            memory,
            argument,
            instruction_length,
            ret_stack,
            section_manager,
            executor_state
        );
    }

    pub fn execute(&mut self) -> Result<(), InstructionError> {
        return (self.instruction_executor)(&mut self.instruction_argument);
    }

    pub fn op_code(&self) -> u16 {
        return self.opcode;
    }
}

impl<'a> InstructionArgument<'a> {
    /// Parse the argument and set the value based on the return value of the closure
    pub fn deref_offset_set<const T: usize>(
        &mut self,
        value: impl FnOnce(&mut InstructionArgument) -> Result<[u8; T], InstructionError>,
    ) -> Result<(), InstructionError> {
        let reg = self.argument.parse_register()?;
        let offset = self.argument.parse_u32()? as usize;
        let is_add = self.argument.parse_boolean()?;
        let value = value(self)?;
        let address = match reg {
            RegisterType::SP => self.register.get_sp(),
            _ => self.register.get_general(&reg)?.into(),
        };
        self.memory.mem_sets(
            inline_if!(is_add, address + offset, address - offset),
            &value,
        )?;
        return Ok(());
    }

    /// Parse the argument assuming it a dereference to a value with `size` input
    pub fn deref_offset_get<const T: usize>(&mut self) -> Result<[u8; T], InstructionError> {
        let reg = self.argument.parse_register()?;
        let offset = self.argument.parse_u32()? as usize;
        let is_add = self.argument.parse_boolean()?;
        let address = match reg {
            RegisterType::SP => self.register.get_sp(),
            _ => self.register.get_general(&reg)?.into(),
        };

        return Ok(self
            .memory
            .mem_gets(inline_if!(is_add, address + offset, address - offset), T)?
            .try_into()
            .unwrap_or([0; T]));
    }

    pub fn parse_section(&mut self) -> Result<&LoadedSection, InstructionError> {
        let section_hash = self.argument.parse_u64()?;
        return Ok(self
            .section_manager
            .get_section_hash(section_hash)
            .ok_or(InstructionError::InvalidSection(section_hash))?);
    }
}
