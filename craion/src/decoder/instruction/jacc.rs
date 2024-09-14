use proc::instruction;

use super::InstructionArgument;

#[instruction(JACC_OPCODE)]
pub fn jacc(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    let reg1 = args.argument.parse_register()?;
    let reg2 = args.argument.parse_register()?;
    let n_reg1 = args.register.get_general(&reg1)?;
    let n_reg2 = args.register.get_general(&reg2)?;
    let (_, overflow) = n_reg1.overflowing_sub(n_reg2);
    if overflow {
        args.register.set_ip(args.argument.parse_address()?);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
