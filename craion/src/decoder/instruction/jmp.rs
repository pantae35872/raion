use proc::instruction;

use super::InstructionArgument;

#[instruction(JMP_OPCODE, "crate::decoder::instruction::jmp::jmp")]
pub fn jmp(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    parse_and_jump!(args);

    let section_hash = args.argument.parse_u64()?;
    let current_section = args
        .section_manager
        .get_section_hash(section_hash)
        .ok_or(super::InstructionError::InvalidSection(section_hash))?;
    args.register
        .set_ip(current_section.mem_start() + args.argument.parse_u16()?.into());
    return Ok(());
}
