use proc::instruction;

use super::InstructionArgument;

#[instruction(JACZ_OPCODE)]
pub fn jacz(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    let reg1 = args.argument.parse_register()?;
    let reg2 = args.argument.parse_register()?;
    let n_reg1 = args.register.get_general(&reg1)?;
    let n_reg2 = args.register.get_general(&reg2)?;
    let (result, _) = n_reg1.overflowing_sub(n_reg2);
    if result == 0 {
        args.register.set_ip(args.argument.parse_address()?);
    } else {
        args.register.inc_ip(args.instruction_length);
    }
    return Ok(());
}
