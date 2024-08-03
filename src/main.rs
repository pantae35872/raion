#![feature(test)]

use decoder::{
    instruction::{ADD_OPCODE, CMP_OPCODE, HALT_OPCODE, INC_OPCODE, JMP_OPCODE, JMZ_OPCODE},
    Decoder,
};
use executor::{registers::RegisterFile, Executor};
use memory::{
    address::Address,
    argument_memory::{self, ArgumentMemory},
};

use crate::memory::Memory;

extern crate test;

pub mod decoder;
pub mod executor;
pub mod memory;

fn main() {
    let mut memory = Memory::new(64);
    let mut instruction_count = 0;
    let opcode = INC_OPCODE.to_le_bytes();
    memory
        .mem_sets(
            Address::new(instruction_count),
            &[4, opcode[0], opcode[1], 4],
        )
        .unwrap();
    instruction_count += 4;
    let opcode = CMP_OPCODE.to_le_bytes();
    memory
        .mem_sets(
            Address::new(instruction_count),
            &[5, opcode[0], opcode[1], 4, 12],
        )
        .unwrap();
    instruction_count += 5;
    let address = Address::new(instruction_count + 22).get_raw().to_le_bytes();
    let opcode = JMZ_OPCODE.to_le_bytes();
    memory
        .mem_sets(
            Address::new(instruction_count),
            &[
                11, opcode[0], opcode[1], address[0], address[1], address[2], address[3],
                address[4], address[5], address[6], address[7],
            ],
        )
        .unwrap();
    instruction_count += 11;
    let address = Address::new(0x0).get_raw().to_le_bytes();
    let opcode = JMP_OPCODE.to_le_bytes();
    memory
        .mem_sets(
            Address::new(instruction_count),
            &[
                11, opcode[0], opcode[1], address[0], address[1], address[2], address[3],
                address[4], address[5], address[6], address[7],
            ],
        )
        .unwrap();
    instruction_count += 11;
    let opcode = HALT_OPCODE.to_le_bytes();
    memory
        .mem_sets(Address::new(instruction_count), &[3, opcode[0], opcode[1]])
        .unwrap();
    let mut register = RegisterFile::new();
    register
        .set_general(&executor::registers::Registers::A64, 0)
        .unwrap();
    register
        .set_general(&executor::registers::Registers::B64, 1)
        .unwrap();
    register
        .set_general(&executor::registers::Registers::C64, 1000000000)
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
}
