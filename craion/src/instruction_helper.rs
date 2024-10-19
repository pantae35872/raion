use common::constants::HALT_OPCODE;

use crate::memory::{address::Address, Memory};

pub struct InstructionHelper<'a> {
    memory: &'a mut Memory,
    write_pos: usize,
}

pub struct InstructionEncoder<'a> {
    opcode: u16,
    instruction_helper: InstructionHelper<'a>,
    args: Vec<u8>,
}

impl<'a> InstructionEncoder<'a> {
    pub fn new(opcode: u16, instruction_helper: InstructionHelper<'a>) -> Self {
        Self {
            opcode,
            instruction_helper,
            args: Vec::new(),
        }
    }

    pub fn encode_sub_opcode(mut self, opcode: u8) -> Self {
        self.args.push(opcode);
        self
    }

    pub fn encode_u64(mut self, value: u64) -> Self {
        self.args.extend_from_slice(&value.to_le_bytes());
        self
    }

    pub fn encode_u32(mut self, value: u32) -> Self {
        self.args.extend_from_slice(&value.to_le_bytes());
        self
    }

    pub fn encode_u16(mut self, value: u16) -> Self {
        self.args.extend_from_slice(&value.to_le_bytes());
        self
    }

    pub fn encode_u8(mut self, value: u8) -> Self {
        self.args.extend_from_slice(&value.to_le_bytes());
        self
    }

    pub fn end(mut self) -> InstructionHelper<'a> {
        let opcode = self.opcode.to_le_bytes();
        let instruction_size = self.args.len() + 3;
        self.instruction_helper
            .memory
            .mem_sets(
                Address::new(self.instruction_helper.write_pos),
                &[instruction_size as u8, opcode[0], opcode[1]],
            )
            .unwrap();
        self.instruction_helper
            .memory
            .mem_sets(
                Address::new(self.instruction_helper.write_pos + 3),
                &self.args,
            )
            .unwrap();
        self.instruction_helper.write_pos += instruction_size;
        self.instruction_helper
    }
}

impl<'a> InstructionHelper<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Self {
            memory,
            write_pos: 0,
        }
    }

    pub fn encode(self, opcode: u16) -> InstructionEncoder<'a> {
        return InstructionEncoder::new(opcode, self);
    }

    pub fn halt(self) -> Self {
        self.encode(HALT_OPCODE).end()
    }
}
