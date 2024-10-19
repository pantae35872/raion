use std::{error::Error, fmt::Display};

use crate::{
    instruction::Instruction,
    executor::ExecutorState,
    memory::{address::Address, argument_memory::ArgumentMemory, Memory, MemoryError}
};

use self::argument::Argument;

pub mod argument;

#[derive(Debug)]
pub enum DecoderError {
    InvalidOpCode(u16),
    InvalidIp(Address),
    InvalidIl(Address, usize)
}

impl Display for DecoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecoderError::InvalidOpCode(opcode) => write!(f, "Trying to decode invalid op code: {}", opcode),
            DecoderError::InvalidIp(ip) => write!(f, "Trying to get instruction length from invalid instruction pointer: {}", ip),
            DecoderError::InvalidIl(ip, il) => 
                write!(f, "Trying to get instruction data from invalid instruction length: {}, with instruction pointer: {}", il, ip)
        }
    }
}

impl Error for DecoderError {}


pub fn decode<'a>(state: &'a mut ExecutorState, argument_memory: &'a mut ArgumentMemory, program_memory: &Memory) -> Result<Instruction<'a>, DecoderError> {
    let instruction_length = match program_memory.mem_get(state.program_state.ip) {
        Ok(il) => il as usize,
        Err(err) => match err {
            MemoryError::InvalidAddr(address) => return Err(DecoderError::InvalidIp(address)),
            MemoryError::OutOfRange(address, _) => return Err(DecoderError::InvalidIp(address))
        },
    };
    let instruction = match program_memory.mem_gets(state.program_state.ip, instruction_length) {
        Ok(is) => is,
        Err(err) => match err {
            MemoryError::InvalidAddr(address) => return Err(DecoderError::InvalidIl(address, instruction_length)),
            MemoryError::OutOfRange(address, _) => 
                return Err(DecoderError::InvalidIl(address, instruction_length))
        },
    };
    if instruction_length < 3 {
        return Err(DecoderError::InvalidIl(state.program_state.ip, instruction_length)); 
    }
    let opcode = u16::from_le_bytes([instruction[1], instruction[2]]);
    let argument = &instruction[3..instruction_length];
    argument_memory.set_arguement(argument);
    return Ok(Instruction::decode(opcode, Argument::new(argument_memory.get_argument()), state, instruction_length)?);
}
