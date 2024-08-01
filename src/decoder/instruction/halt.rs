use crate::{decoder::argument::Argument, executor::registers::RegisterFile, memory::Memory};

use super::{Instruction, HALT_OPCODE};

pub struct Halt<'a> {
    register: &'a mut RegisterFile,
    instruction_length: usize,
}

impl<'a> Halt<'a> {
    pub fn new(register: &'a mut RegisterFile, instruction_length: usize) -> Self {
        Self {
            register,
            instruction_length,
        }
    }
}

impl<'a> Instruction for Halt<'a> {
    fn execute(&mut self) -> Result<(), super::InstructionError> {
        self.register.inc_ip(self.instruction_length);
        self.register.set_halt(true);
        return Ok(());
    }

    fn op_code(&self) -> u16 {
        return HALT_OPCODE;
    }
}
