use proc::instruction;

use super::InstructionArgument;

#[instruction(HALT_OPCODE)]
pub fn halt(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    args.register.set_halt(true);
    return Ok(());
}
