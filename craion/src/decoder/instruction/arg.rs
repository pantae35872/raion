use common::constants::{ARG_NUM, ARG_OPCODE, ARG_REG};
use proc::instruction;

use super::InstructionArgument;

#[instruction(ARG_OPCODE, "crate::decoder::instruction::arg::arg")]
pub fn arg(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);

    match args.argument.parse_u8()? {
        ARG_NUM => args
            .executor_state
            .load_argument(args.argument.parse_u32()?, args.argument.parse_u64()?),
        ARG_REG => args.executor_state.load_argument(
            args.argument.parse_u32()?,
            args.register
                .get_general(&args.argument.parse_register()?)?,
        ),
        invalid_subop_code => {
            return Err(super::InstructionError::InvalidSubOpCode(
                ARG_OPCODE,
                invalid_subop_code,
            ));
        }
    };
    return Ok(());
}
