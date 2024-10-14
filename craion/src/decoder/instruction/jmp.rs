use proc::instruction;

use super::InstructionArgument;

#[instruction(JMP_OPCODE, "crate::decoder::instruction::jmp::jmp")]
pub fn jmp(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    parse_and_jump!(args);
    return Ok(());
}
