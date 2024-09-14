use proc::instruction;

use super::InstructionArgument;

#[instruction(OUTC_OPCODE)]
pub fn outc(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let reg = args.argument.parse_register()?;
    let value = char::from_u32(args.register.get_general(&reg)? as u32)
        .ok_or(super::InstructionError::InvalidUTF8)?;
    print!("{value}");
    return Ok(());
}
