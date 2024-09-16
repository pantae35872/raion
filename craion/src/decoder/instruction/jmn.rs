use proc::instruction;

use super::InstructionArgument;

#[instruction(JMN_OPCODE)]
pub fn jmn(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    if args.register.get_negative() {
        parse_and_jump!(args);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
