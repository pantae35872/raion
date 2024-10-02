use proc::instruction;

use super::InstructionArgument;

#[instruction(LARG_OPCODE, "crate::decoder::instruction::larg::larg")]
pub fn larg(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let reg = args.argument.parse_register()?;
    let index = args.argument.parse_u32()?;
    args.register
        .set_general(&reg, args.executor_state.get_argument(index))?;
    return Ok(());
}
