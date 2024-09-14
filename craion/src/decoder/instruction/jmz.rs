use proc::instruction;

use super::InstructionArgument;

#[instruction(JMZ_OPCODE)]
pub fn jmz(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    if args.register.get_zero() {
        args.register.set_ip(args.argument.parse_address()?);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
