use crate::{decoder::argument::Argument, executor::registers::RegisterFile};

use super::{Instruction, JACN_OPCODE};

pub struct Jacn<'a, 'b> {
    register: &'a mut RegisterFile,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Jacn<'a, 'b> {
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

impl<'a, 'b> Instruction for Jacn<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        let reg1 = self.argument.parse_register()?;
        let reg2 = self.argument.parse_register()?;
        let n_reg1 = self.register.get_general(&reg1)?;
        let n_reg2 = self.register.get_general(&reg2)?;
        let (result, _) = n_reg1.overflowing_sub(n_reg2);
        if result & (0b1u64 << 63) != 0 {
            self.register.set_ip(self.argument.parse_address()?);
        } else {
            self.register.inc_ip(self.instruction_length);
        }
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return JACN_OPCODE;
    }
}
