#![deny(warnings)]
#![feature(test)]

use craion::executor::{registers::RegisterFile, Executor};
use craion::memory::{address::Address, argument_memory::ArgumentMemory};

use craion::memory::{Memory, MemoryError};

extern crate test;

fn program(memory: &mut Memory) -> Result<(), MemoryError> {
    memory.mem_sets(
        Address::new(0),
        &[
            12, 16, 0, 4, 254, 255, 0, 0, 0, 0, 0, 0, 13, 16, 0, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 13,
            16, 0, 3, 8, 0, 202, 154, 59, 0, 0, 0, 0, 4, 30, 0, 4, 13, 67, 0, 4, 8, 38, 0, 0, 0, 0,
            0, 0, 0, 3, 255, 255,
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
