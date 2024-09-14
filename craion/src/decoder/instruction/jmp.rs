use proc::instruction;

use super::InstructionArgument;

#[instruction(JMP_OPCODE)]
pub fn jmp(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.set_ip(args.argument.parse_address()?);
    return Ok(());
}
