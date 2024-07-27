use executor::Executor;

use crate::memory::Memory;

pub mod decoder;
pub mod executor;
pub mod memory;

fn main() {
    let mut memory = Memory::new(16);
    let mut executor = Executor::new(&mut memory);
    executor.tick();
}
