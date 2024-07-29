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
        let n_reg1 = self.register.get_general(&reg1)?;
        let n_reg2 = self.register.get_general(&reg2)?;
        let (result, overflow) = n_reg1.overflowing_add(n_reg2);
        self.register.set_carry(overflow);
        self.register.set_zero(result == 0);
        self.register.set_negative(result & (0b1u64 << 63) != 0);
        self.register.set_general(&reg1, result)?;
        return Ok(());
    }
}
