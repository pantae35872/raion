use proc::instruction;

use super::InstructionArgument;

#[instruction(JMZ_OPCODE, "crate::decoder::instruction::jmz::jmz")]
pub fn jmz(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    if args.register.get_zero() {
        parse_and_jump!(args);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
