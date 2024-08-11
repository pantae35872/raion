use crate::{decoder::argument::Argument, executor::registers::RegisterFile};

use super::{Instruction, JACE_OPCODE};

pub struct Jace<'a, 'b> {
    register: &'a mut RegisterFile,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Jace<'a, 'b> {
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

impl<'a, 'b> Instruction for Jace<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        let reg1 = self.argument.parse_register()?;
        let reg2 = self.argument.parse_register()?;
        let n_reg1 = self.register.get_general(&reg1)?;
        let n_reg2 = self.register.get_general(&reg2)?;
        let (result, overflow) = n_reg1.overflowing_sub(n_reg2);
        if !overflow && result != 0 && result & (0b1u64 << 63) == 0 {
            self.register.set_ip(self.argument.parse_address()?);
        } else {
            self.register.inc_ip(self.instruction_length);
        }
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return JACE_OPCODE;
    }
}
