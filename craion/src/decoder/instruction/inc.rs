use proc::instruction;

use super::InstructionArgument;

#[instruction(INC_OPCODE, "crate::decoder::instruction::inc::inc")]
pub fn inc(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let reg = args.argument.parse_register()?;
    let (result, overflow) = args.register.get_general(&reg)?.overflowing_add(1);
    args.register.set_general(&reg, result)?;
    args.register.set_carry(overflow);
    args.register.set_zero(result == 0);
    args.register.set_negative(result & (0b1u64 << 63) != 0);
    return Ok(());
}
