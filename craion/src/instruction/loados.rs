use proc::instruction;

use crate::{decoder::argument::Argument, executor::ExecutorState};

#[instruction(LOADOS_OPCODE, "crate::instruction::loados::loados")]
pub fn loados(
    state: &mut ExecutorState,
    argument: &mut Argument,
    instruction_length: usize,
) -> Result<(), super::InstructionError> {
    state.program_state.inc_ip(instruction_length);
    let index = argument.parse_u16()?;
    let object = state.operand_stack.pop();
    state.program_state.local.set(index as usize, object);
    return Ok(());
}
