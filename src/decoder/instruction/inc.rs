use crate::{decoder::argument::Argument, executor::registers::RegisterFile};

use super::{Instruction, INC_OPCODE};

pub struct Inc<'a, 'b> {
    register: &'a mut RegisterFile,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Inc<'a, 'b> {
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

impl<'a, 'b> Instruction for Inc<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.inc_ip(self.instruction_length);
        let reg = self.argument.parse_register()?;
        let (result, overflow) = self.register.get_general(&reg)?.overflowing_add(1);
        self.register.set_general(&reg, result)?;
        self.register.set_carry(overflow);
        self.register.set_zero(result == 0);
        self.register.set_negative(result & (0b1u64 << 63) != 0);
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return INC_OPCODE;
    }
}
