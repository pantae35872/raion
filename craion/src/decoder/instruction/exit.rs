use proc::instruction;

use super::InstructionArgument;

#[instruction(EXIT_OPCODE)]
pub fn exit(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    args.register.set_halt(true);
    let reg = args.argument.parse_register()?;
    args.executor_state
        .set_exit_code(args.register.get_general(&reg)?);
    return Ok(());
}
