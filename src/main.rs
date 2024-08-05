#![feature(test)]

use craion::decoder::instruction::mov::MOV_REG2REG;
use craion::decoder::instruction::{
    ADD_OPCODE, CMP_OPCODE, INC_OPCODE, JACN_OPCODE, JMN_OPCODE, JMP_OPCODE, JMZ_OPCODE, MOV_OPCODE,
};
use craion::executor::registers::Registers;
use craion::executor::{registers::RegisterFile, Executor};
use craion::instruction_helper::InstructionHelper;
use craion::memory::{address::Address, argument_memory::ArgumentMemory};

use craion::memory::{Memory, MemoryError};

extern crate test;

fn program(memory: &mut Memory) -> Result<(), MemoryError> {
    let mut arg = [4, 12].to_vec();
    arg.extend_from_slice(&Address::new(0).get_raw().to_le_bytes());
    InstructionHelper::new(memory)
        .encode(INC_OPCODE, &[4])?
        .encode(JACN_OPCODE, &arg)?
        .halt()?;
    return Ok(());
}

fn main() {
    let mut memory = Memory::new(64);
    program(&mut memory).unwrap();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 0).unwrap();
    register.set_general(&Registers::B64, 1).unwrap();
    register.set_general(&Registers::C64, 1000000).unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    println!("{:?}", register);
}
