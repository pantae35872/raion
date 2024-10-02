use proc::instruction;

use super::InstructionArgument;

#[instruction(JMC_OPCODE, "crate::decoder::instruction::jmc::jmc")]
pub fn jmc(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    if args.register.get_carry() {
        parse_and_jump!(args);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
