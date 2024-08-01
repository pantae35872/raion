use crate::{decoder::argument::Argument, executor::registers::RegisterFile, memory::Memory};

use super::{Instruction, MOV_OPCODE};

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
            1 => {
                self.register.set_general(
                    &self.argument.parse_register()?,
                    self.register
                        .get_general(&self.argument.parse_register()?)?,
                )?;
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
