use crate::{
    decoder::argument::Argument,
    executor::registers::{RegisterFile, RegisterSizes},
    memory::Memory,
};

use super::{Instruction, MOV_OPCODE};

pub const MOV_REG2REG: u8 = 1;
pub const MOV_REG2MEM: u8 = 2;

pub struct Mov<'a, 'b> {
    register: &'a mut RegisterFile,
    memory: &'b mut Memory,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Mov<'a, 'b> {
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

impl<'a, 'b> Instruction for Mov<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.inc_ip(self.instruction_length);
        match self.argument.parse_u8()? {
            MOV_REG2REG => {
                self.register.set_general(
                    &self.argument.parse_register()?,
                    self.register
                        .get_general(&self.argument.parse_register()?)?,
                )?;
            }
            MOV_REG2MEM => {
                let address = self.argument.parse_address()?;
                let register = self.argument.parse_register()?;
                let value = self.register.get_general(&register)?;
                match register.size() {
                    RegisterSizes::SizeU8 => {
                        self.memory.mem_set(address, value as u8)?;
                    }
                    RegisterSizes::SizeU16 => {
                        self.memory
                            .mem_sets(address, &(value as u16).to_le_bytes())?;
                    }
                    RegisterSizes::SizeU32 => {
                        self.memory
                            .mem_sets(address, &(value as u32).to_le_bytes())?;
                    }
                    RegisterSizes::SizeU64 => {
                        self.memory
                            .mem_sets(address, &(value as u64).to_le_bytes())?;
                    }
                    RegisterSizes::SizeBool => unreachable!(),
                }
            }
            invalid_subop_code => {
                return Err(super::InstructionError::InvalidSubOpCode(
                    MOV_OPCODE,
                    invalid_subop_code,
                ))
            }
        };
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return MOV_OPCODE;
    }
}
