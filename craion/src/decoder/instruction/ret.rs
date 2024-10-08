use proc::instruction;

use super::{InstructionArgument, InstructionError};

#[instruction(RET_OPCODE, "crate::decoder::instruction::ret::ret")]
pub fn ret(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.set_ip(
        args.ret_stack
            .pop()
            .ok_or(InstructionError::EmptyRetStack)?,
    );
    return Ok(());
}
