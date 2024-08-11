use crate::{
    decoder::argument::Argument,
    executor::registers::{RegisterFile, RegisterSizes},
    memory::Memory,
};

use super::{Instruction, PUSH_OPCODE};

pub struct Push<'a, 'b> {
    register: &'a mut RegisterFile,
    memory: &'b mut Memory,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Push<'a, 'b> {
    pub fn new(
        register: &'a mut RegisterFile,
        memory: &'b mut Memory,
        argument: Argument<'b>,
        instruction_length: usize,
    ) -> Self {
        Self {
            register,
            memory,
            argument,
            instruction_length,
        }
    }
}

impl<'a, 'b> Instruction for Push<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.inc_ip(self.instruction_length);
        let reg = self.argument.parse_register()?;
        let value = self.register.get_general(&reg)?;
        let sp = self.register.dec_sp(reg.size().byte());
        match reg.size() {
            RegisterSizes::SizeU8 => {
                self.memory.mem_set(sp, value as u8)?;
            }
            RegisterSizes::SizeU16 => {
                self.memory.mem_sets(sp, &(value as u16).to_le_bytes())?;
            }
            RegisterSizes::SizeU32 => {
                self.memory.mem_sets(sp, &(value as u32).to_le_bytes())?;
            }
            RegisterSizes::SizeU64 => {
                self.memory.mem_sets(sp, &(value as u64).to_le_bytes())?;
            }
            RegisterSizes::SizeBool => unreachable!(),
        }
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return PUSH_OPCODE;
    }
}
