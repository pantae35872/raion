use proc::instruction;

use super::InstructionArgument;

#[instruction(ENTER_OPCODE, "crate::decoder::instruction::enter::enter")]
pub fn enter(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let size = args.argument.parse_u64()?;
    args.register.set_sp(args.register.get_sp() - size as usize);
    args.executor_state.save_stack_size(size);
    return Ok(());
}
