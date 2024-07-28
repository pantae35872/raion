use crate::{decoder::argument::Argument, executor::registers::RegisterFile, memory::Memory};

use super::Instruction;

pub struct Add<'a, 'b> {
    register: &'a mut RegisterFile,
    memory: &'b mut Memory,
    argument: Argument<'b>,
    instruction_length: usize,
}

impl<'a, 'b> Add<'a, 'b> {
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

impl<'a, 'b> Instruction for Add<'a, 'b> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.inc_ip(self.instruction_length);
        let reg1 = self.argument.parse_register()?;
        let reg2 = self.argument.parse_register()?;
        self.register.set_general(
            &reg1,
            self.register.get_general(&reg1)? + self.register.get_general(&reg2)?,
        )?;
        return Ok(());
    }
}
