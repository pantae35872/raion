use common::constants::HALT_OPCODE;

use craion::memory::{address::Address, Memory, MemoryError};

pub struct InstructionHelper<'a> {
    memory: &'a mut Memory,
    write_pos: usize,
}

impl<'a> InstructionHelper<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Self {
            memory,
            write_pos: 0,
        }
    }

    pub fn encode(&mut self, opcode: u16, argument: &[u8]) -> Result<&mut Self, MemoryError> {
        let opcode = opcode.to_le_bytes();
        let instruction_size = argument.len() + 3;
        self.memory.mem_sets(
            Address::new(self.write_pos),
            &[instruction_size as u8, opcode[0], opcode[1]],
        )?;
        self.memory
            .mem_sets(Address::new(self.write_pos + 3), argument)?;
        self.write_pos += instruction_size;
        return Ok(self);
    }

    pub fn halt(&mut self) -> Result<(), MemoryError> {
        self.encode(HALT_OPCODE, &[])?;
        return Ok(());
    }
}
