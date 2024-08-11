#![feature(test)]

use craion::decoder::instruction::{INC_OPCODE, JACN_OPCODE};
use craion::executor::registers::Registers;
use craion::executor::{registers::RegisterFile, Executor};
use craion::instruction_helper::InstructionHelper;
use craion::memory::{address::Address, argument_memory::ArgumentMemory};

use craion::memory::{Memory, MemoryError};

extern crate test;

fn program(memory: &mut Memory) -> Result<(), MemoryError> {
    memory.mem_sets(
        Address::new(0),
        &[
            13, 16, 0, 3, 8, 0, 202, 154, 59, 0, 0, 0, 0, 12, 16, 0, 4, 254, 255, 0, 0, 0, 0, 0, 0,
            4, 17, 0, 8, 13, 16, 0, 3, 8, 254, 0, 0, 0, 0, 0, 0, 0, 4, 18, 0, 8, 3, 255, 255,
        ],
    )?;
    return Ok(());
}

fn main() {
    let mut memory = Memory::new(0xFFFF);
    program(&mut memory).unwrap();
    let mut register = RegisterFile::new();
    let mut argument_memory = ArgumentMemory::new();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    println!("{:?}", register);
}
