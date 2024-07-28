use crate::{
    decoder::argument::{self, Argument},
    executor::registers::RegisterFile,
};

use super::Instruction;

pub struct Mov<'a, 'b> {
    register: &'a mut RegisterFile,
    argument: &'b mut Argument<'b>,
}

impl<'a, 'b> Mov<'a, 'b> {
    pub fn new(register: &'a mut RegisterFile, argument: &'b mut Argument<'b>) -> Self {
        Self { register, argument }
    }
}

impl<'a, 'b> Instruction for Mov<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        return Ok(self.register.set_general(
            self.argument.parse_register()?,
            self.register.get_general(self.argument.parse_register()?)?,
        )?);
    }
}
