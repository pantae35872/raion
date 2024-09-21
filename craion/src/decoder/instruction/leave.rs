use proc::instruction;

use super::InstructionArgument;

#[instruction(LEAVE_OPCODE)]
pub fn leave(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let size = args.executor_state.consume_stack_size();
    args.register.set_sp(args.register.get_sp() + size as usize);
    return Ok(());
}
