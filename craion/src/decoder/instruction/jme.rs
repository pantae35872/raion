use proc::instruction;

use super::InstructionArgument;

#[instruction(JME_OPCODE)]
pub fn jme(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    if !(args.register.get_negative() || args.register.get_zero() || args.register.get_carry()) {
        args.register.set_ip(args.argument.parse_address()?);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
