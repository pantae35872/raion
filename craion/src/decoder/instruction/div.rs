use proc::instruction;

use super::InstructionArgument;

#[instruction(DIV_OPCODE, "crate::decoder::instruction::div::div")]
pub fn div(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let reg1 = args.argument.parse_register()?;
    let reg2 = args.argument.parse_register()?;
    let n_reg1 = args.register.get_general(&reg1)?;
    let n_reg2 = args.register.get_general(&reg2)?;
    let (result, overflow) = n_reg1.overflowing_div(n_reg2);
    args.register.set_carry(overflow);
    args.register.set_zero(result == 0);
    args.register.set_negative(result & (0b1u64 << 63) != 0);
    args.register.set_general(&reg1, result)?;
    return Ok(());
}
