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
        self.register
            .set_general(&reg, self.register.get_general(&reg)? + 1)?;
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return INC_OPCODE;
    }
}
