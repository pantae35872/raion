use proc::instruction;

use crate::{decoder::argument::Argument, executor::ExecutorState};

#[instruction(LOCAL_OPCODE, "crate::instruction::local::local")]
pub fn local(
    state: &mut ExecutorState,
    argument: &mut Argument,
    instruction_length: usize,
) -> Result<(), super::InstructionError> {
    state.program_state.inc_ip(instruction_length);
    let size = argument.parse_u16()?;
    state.program_state.local.new_local(size.into());
    return Ok(());
}
