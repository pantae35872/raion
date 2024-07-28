use executor::{registers::RegisterFile, Executor};
use memory::address::Address;

use crate::memory::Memory;

pub mod decoder;
pub mod executor;
pub mod memory;

fn main() {
    let mut memory = Memory::new(16);
    let opcode = 16u16.to_le_bytes();
    memory
        .mem_sets(Address::new(0x0), &[6, opcode[0], opcode[1], 1, 1, 5])
        .unwrap();
    let opcode = 32u16.to_le_bytes();
    memory
        .mem_sets(Address::new(0x6), &[5, opcode[0], opcode[1], 1, 5])
        .unwrap();
    let opcode = 65535u16.to_le_bytes();
    memory
        .mem_sets(Address::new(0xb), &[3, opcode[0], opcode[1]])
        .unwrap();
    let mut register = RegisterFile::new();
    register
        .set_general(&executor::registers::Registers::A8, 5)
        .unwrap();
    register
        .set_general(&executor::registers::Registers::B8, 8)
        .unwrap();
    let mut executor = Executor::new(&mut memory, &mut register);
    executor.debug_register();
    executor.execute();
}
