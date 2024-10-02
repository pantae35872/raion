use proc::instruction;

use super::InstructionArgument;

#[instruction(JME_OPCODE, "crate::decoder::instruction::jme::jme")]
pub fn jme(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    if !(args.register.get_negative() || args.register.get_zero() || args.register.get_carry()) {
        parse_and_jump!(args);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
