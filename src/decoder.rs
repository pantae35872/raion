use std::{error::Error, fmt::Display};

use instruction::Instructions;

use crate::{
    executor::registers::RegisterFile,
    memory::{address::Address, argument_memory::ArgumentMemory, Memory, MemoryError},
};

use self::argument::Argument;

pub mod instruction;
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

pub struct Decoder<'a, 'b> {
    memory: &'a mut Memory,
    argument_memory: &'a mut ArgumentMemory,
    register: &'b mut RegisterFile,
}

impl<'a, 'b> Decoder<'a, 'b> {
    pub fn new(memory: &'a mut Memory, register: &'b mut RegisterFile, argument_memory: &'a mut ArgumentMemory) -> Self {
        Self { memory, register, argument_memory }
    }

    pub fn decode(&mut self) -> Result<Instructions, DecoderError> {
        let instruction_length = match self.memory.mem_get(self.register.get_ip()) {
            Ok(il) => il as usize,
            Err(err) => match err {
                MemoryError::InvalidAddr(address) => return Err(DecoderError::InvalidIp(address)),
                MemoryError::OutOfRange(address, _) => return Err(DecoderError::InvalidIp(address))
            },
        };
        let instruction = match self.memory.mem_gets(self.register.get_ip(), instruction_length) {
            Ok(is) => is,
            Err(err) => match err {
                MemoryError::InvalidAddr(address) => return Err(DecoderError::InvalidIl(address, instruction_length)),
                MemoryError::OutOfRange(address, _) => 
                    return Err(DecoderError::InvalidIl(address, instruction_length))
            },
        };
        if instruction_length < 3 {
            return Err(DecoderError::InvalidIl(self.register.get_ip(), instruction_length)); 
        }
        let opcode = u16::from_le_bytes(<[u8; 2]>::try_from(&instruction[1..=2]).unwrap());
        let argument = &instruction[3..instruction_length];
        self.argument_memory.set_arguement(argument);
        return Ok(Instructions::decode(opcode, self.register, self.memory, Argument::new(self.argument_memory.get_argument()), instruction_length)?);
    }
}
