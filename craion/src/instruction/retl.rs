use proc::instruction;

use crate::{decoder::argument::Argument, executor::ExecutorState};

#[instruction(RETL_OPCODE, "crate::instruction::retl::retl")]
pub fn retl(
    state: &mut ExecutorState,
    argument: &mut Argument,
    instruction_length: usize,
) -> Result<(), super::InstructionError> {
    state.program_state.inc_ip(instruction_length);
    let index = argument.parse_u16()?;
    state.return_stack.ret(&mut state.program_state);
    return Ok(());
}
