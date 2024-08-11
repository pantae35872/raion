use crate::{
    decoder::argument::Argument,
    executor::registers::{RegisterFile, RegisterSizes},
    memory::Memory,
};

use super::{Instruction, POP_OPCODE};

pub struct Pop<'a, 'b> {
    register: &'a mut RegisterFile,
    memory: &'b mut Memory,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Pop<'a, 'b> {
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

impl<'a, 'b> Instruction for Pop<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.inc_ip(self.instruction_length);
        let reg = self.argument.parse_register()?;
        let sp = self.register.get_sp();
        match reg.size() {
            RegisterSizes::SizeU8 => {
                self.register
                    .set_general(&reg, self.memory.mem_get(sp)? as u64)?;
            }
            RegisterSizes::SizeU16 => {
                self.register.set_general(
                    &reg,
                    u16::from_le_bytes(<[u8; 2]>::try_from(self.memory.mem_gets(sp, 2)?).unwrap())
                        .into(),
                )?;
            }
            RegisterSizes::SizeU32 => {
                self.register.set_general(
                    &reg,
                    u32::from_le_bytes(<[u8; 4]>::try_from(self.memory.mem_gets(sp, 4)?).unwrap())
                        .into(),
                )?;
            }
            RegisterSizes::SizeU64 => {
                self.register.set_general(
                    &reg,
                    u64::from_le_bytes(<[u8; 8]>::try_from(self.memory.mem_gets(sp, 8)?).unwrap()),
                )?;
            }
            RegisterSizes::SizeBool => unreachable!(),
        }
        self.register.inc_sp(reg.size().byte());
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return POP_OPCODE;
    }
}
