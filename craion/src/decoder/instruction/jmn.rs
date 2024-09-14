use proc::instruction;

use super::InstructionArgument;

#[instruction(JMN_OPCODE)]
pub fn jmn(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    if args.register.get_negative() {
        args.register.set_ip(args.argument.parse_address()?);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
