use crate::{decoder::argument::Argument, executor::registers::RegisterFile};

use super::{Instruction, SUB_OPCODE};

pub struct Sub<'a, 'b> {
    register: &'a mut RegisterFile,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Sub<'a, 'b> {
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

impl<'a, 'b> Instruction for Sub<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.inc_ip(self.instruction_length);
        let reg1 = self.argument.parse_register()?;
        let reg2 = self.argument.parse_register()?;
        let n_reg1 = self.register.get_general(&reg1)?;
        let n_reg2 = self.register.get_general(&reg2)?;
        let (result, overflow) = n_reg1.overflowing_sub(n_reg2);
        self.register.set_carry(overflow);
        self.register.set_zero(result == 0);
        self.register.set_negative(result & (0b1u64 << 63) != 0);
        self.register.set_general(&reg1, result)?;
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return SUB_OPCODE;
    }
}
