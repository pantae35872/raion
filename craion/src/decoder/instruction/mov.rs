use common::constants::{
    MOV_ADD2SP, MOV_NUM2REG, MOV_OPCODE, MOV_REG2MEM, MOV_REG2REG, MOV_REG2SP,
};

use crate::{
    decoder::argument::Argument,
    executor::registers::{RegisterFile, RegisterSizes},
    memory::{address::Address, Memory},
};

use super::Instruction;

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
            MOV_NUM2REG => {
                let reg = self.argument.parse_register()?;
                self.register.set_general(
                    &reg,
                    match reg.size() {
                        RegisterSizes::SizeU8 => self.argument.parse_u8()?.into(),
                        RegisterSizes::SizeU16 => self.argument.parse_u16()?.into(),
                        RegisterSizes::SizeU32 => self.argument.parse_u32()?.into(),
                        RegisterSizes::SizeU64 => self.argument.parse_u64()?.into(),
                        RegisterSizes::SizeBool => 0,
                    },
                )?;
            }
            MOV_ADD2SP => {
                self.register.set_sp(self.argument.parse_address()?);
            }
            MOV_REG2SP => {
                self.register.set_sp(Address::new(
                    self.register
                        .get_general(&self.argument.parse_register()?)?
                        as usize,
                ));
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
