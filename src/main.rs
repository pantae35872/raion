use executor::{registers::Register, Executor};
use memory::address::Address;

use crate::memory::Memory;

pub mod decoder;
pub mod executor;
pub mod memory;

fn main() {
    let mut memory = Memory::new(16);
    let opcode = 17u16.to_le_bytes();
    memory.mem_sets(Address::new(0x0), &[4, opcode[0], opcode[1], 1]);
    let mut register = Register::new();
    let mut executor = Executor::new(&mut memory, &mut register);
    executor.execute();
}
