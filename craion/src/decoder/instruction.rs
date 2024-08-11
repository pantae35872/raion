use std::{error::Error, fmt::Display};

use cmp::Cmp;
use common::constants::{
    ADD_OPCODE, CMP_OPCODE, HALT_OPCODE, INC_OPCODE, JACC_OPCODE, JACE_OPCODE, JACN_OPCODE,
    JACZ_OPCODE, JMC_OPCODE, JME_OPCODE, JMN_OPCODE, JMP_OPCODE, JMZ_OPCODE, MOV_OPCODE,
    POP_OPCODE, PUSH_OPCODE, SUB_OPCODE,
};
use inc::Inc;
use jacc::Jacc;
use jace::Jace;
use jacn::Jacn;
use jacz::Jacz;
use jmc::Jmc;
use jme::Jme;
use jmn::Jmn;
use jmp::Jmp;
use jmz::Jmz;
use pop::Pop;
use push::Push;
use sub::Sub;

use crate::{
    executor::registers::{RegisterFile, RegisterFileError},
    memory::{Memory, MemoryError},
};

use self::{add::Add, halt::Halt, mov::Mov};

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

pub enum Instructions<'a> {
    Mov(Mov<'a, 'a>),
    Push(Push<'a, 'a>),
    Pop(Pop<'a, 'a>),
    Add(Add<'a, 'a>),
    Halt(Halt<'a>),
    Jmp(Jmp<'a, 'a>),
    Jmz(Jmz<'a, 'a>),
    Cmp(Cmp<'a, 'a>),
    Inc(Inc<'a, 'a>),
    Jmn(Jmn<'a, 'a>),
    Sub(Sub<'a, 'a>),
    Jacn(Jacn<'a, 'a>),
    Jacz(Jacz<'a, 'a>),
    Jacc(Jacc<'a, 'a>),
    Jace(Jace<'a, 'a>),
    Jme(Jme<'a, 'a>),
    Jmc(Jmc<'a, 'a>),
}

impl<'a> Instructions<'a> {
    pub fn decode(
        op_code: u16,
        register: &'a mut RegisterFile,
        memory: &'a mut Memory,
        argument: Argument<'a>,
        instruction_length: usize,
    ) -> Result<Self, DecoderError> {
        match op_code {
            MOV_OPCODE => {
                return Ok(Self::Mov(Mov::new(
                    register,
                    memory,
                    argument,
                    instruction_length,
                )))
            }
            PUSH_OPCODE => {
                return Ok(Self::Push(Push::new(
                    register,
                    memory,
                    argument,
                    instruction_length,
                )))
            }
            POP_OPCODE => {
                return Ok(Self::Pop(Pop::new(
                    register,
                    memory,
                    argument,
                    instruction_length,
                )))
            }
            ADD_OPCODE => return Ok(Self::Add(Add::new(register, argument, instruction_length))),
            SUB_OPCODE => return Ok(Self::Sub(Sub::new(register, argument, instruction_length))),
            HALT_OPCODE => return Ok(Self::Halt(Halt::new(register, instruction_length))),
            JMP_OPCODE => return Ok(Self::Jmp(Jmp::new(register, argument))),
            JMN_OPCODE => return Ok(Self::Jmn(Jmn::new(register, argument, instruction_length))),
            JMZ_OPCODE => return Ok(Self::Jmz(Jmz::new(register, argument, instruction_length))),
            JMC_OPCODE => return Ok(Self::Jmc(Jmc::new(register, argument, instruction_length))),
            JACN_OPCODE => {
                return Ok(Self::Jacn(Jacn::new(
                    register,
                    argument,
                    instruction_length,
                )))
            }
            JACZ_OPCODE => {
                return Ok(Self::Jacz(Jacz::new(
                    register,
                    argument,
                    instruction_length,
                )))
            }
            JACC_OPCODE => {
                return Ok(Self::Jacc(Jacc::new(
                    register,
                    argument,
                    instruction_length,
                )))
            }
            JACE_OPCODE => {
                return Ok(Self::Jace(Jace::new(
                    register,
                    argument,
                    instruction_length,
                )))
            }
            JME_OPCODE => return Ok(Self::Jme(Jme::new(register, argument, instruction_length))),
            CMP_OPCODE => return Ok(Self::Cmp(Cmp::new(register, argument, instruction_length))),
            INC_OPCODE => return Ok(Self::Inc(Inc::new(register, argument, instruction_length))),
            iop_code => return Err(DecoderError::InvalidOpCode(iop_code)),
        }
    }
}

impl<'a> Instruction for Instructions<'a> {
    fn execute(&mut self) -> Result<(), InstructionError> {
        match self {
            Self::Mov(mov) => mov.execute(),
            Self::Push(push) => push.execute(),
            Self::Pop(pop) => pop.execute(),
            Self::Add(add) => add.execute(),
            Self::Sub(sub) => sub.execute(),
            Self::Halt(halt) => halt.execute(),
            Self::Jmp(jmp) => jmp.execute(),
            Self::Jmn(jmn) => jmn.execute(),
            Self::Jmc(jmc) => jmc.execute(),
            Self::Jmz(jmz) => jmz.execute(),
            Self::Cmp(cmp) => cmp.execute(),
            Self::Inc(inc) => inc.execute(),
            Self::Jacn(jacn) => jacn.execute(),
            Self::Jacz(jacz) => jacz.execute(),
            Self::Jacc(jacc) => jacc.execute(),
            Self::Jme(jme) => jme.execute(),
            Self::Jace(jace) => jace.execute(),
        }
    }

    fn op_code(&self) -> u16 {
        match self {
            Self::Mov(mov) => mov.op_code(),
            Self::Push(push) => push.op_code(),
            Self::Pop(pop) => pop.op_code(),
            Self::Add(add) => add.op_code(),
            Self::Sub(sub) => sub.op_code(),
            Self::Halt(halt) => halt.op_code(),
            Self::Jmp(jmp) => jmp.op_code(),
            Self::Jmn(jmn) => jmn.op_code(),
            Self::Jme(jme) => jme.op_code(),
            Self::Jmz(jmz) => jmz.op_code(),
            Self::Jmc(jmc) => jmc.op_code(),
            Self::Cmp(cmp) => cmp.op_code(),
            Self::Inc(inc) => inc.op_code(),
            Self::Jacn(jacn) => jacn.op_code(),
            Self::Jacz(jacz) => jacz.op_code(),
            Self::Jacc(jacc) => jacc.op_code(),
            Self::Jace(jace) => jace.op_code(),
        }
    }
}

pub trait Instruction {
    fn execute(&mut self) -> Result<(), InstructionError>;
    fn op_code(&self) -> u16;
}
