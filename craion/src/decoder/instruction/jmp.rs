use crate::{decoder::argument::Argument, executor::registers::RegisterFile};

use super::{Instruction, JMP_OPCODE};

pub struct Jmp<'a, 'b> {
    register: &'a mut RegisterFile,
    argument: Argument<'b>,
}

impl<'a, 'b> Jmp<'a, 'b> {
    pub fn new(register: &'a mut RegisterFile, argument: Argument<'b>) -> Self {
        Self { register, argument }
    }
}

impl<'a, 'b> Instruction for Jmp<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.set_ip(self.argument.parse_address()?);
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return JMP_OPCODE;
    }
}
