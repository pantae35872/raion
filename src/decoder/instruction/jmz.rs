use crate::{decoder::argument::Argument, executor::registers::RegisterFile, memory::Memory};

use super::{Instruction, JMZ_OPCODE};

pub struct Jmz<'a, 'b> {
    register: &'a mut RegisterFile,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Jmz<'a, 'b> {
    pub fn new(
        register: &'a mut RegisterFile,
        argument: Argument<'b>,
        instruction_length: usize,
    ) -> Self {
        Self {
            register,
            argument,
            instruction_length,
        }
    }
}

impl<'a, 'b> Instruction for Jmz<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        if self.register.get_zero() {
            self.register.set_ip(self.argument.parse_address()?);
        } else {
            self.register.inc_ip(self.instruction_length);
        }
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return JMZ_OPCODE;
    }
}
