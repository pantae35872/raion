use common::constants::{ADD_OPCODE, ADD_REG_W_NUM, ADD_REG_W_REG, ADD_SP_W_NUM};
use proc::instruction;

use crate::memory::address::Address;

use super::InstructionArgument;

#[instruction(ADD_OPCODE)]
pub fn add(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);

    match args.argument.parse_u8()? {
        ADD_REG_W_REG => {
            let reg1 = args.argument.parse_register()?;
            let reg2 = args.argument.parse_register()?;
            let n_reg1 = args.register.get_general(&reg1)?;
            let n_reg2 = args.register.get_general(&reg2)?;
            let (result, overflow) = n_reg1.overflowing_add(n_reg2);
            args.register.set_carry(overflow);
            args.register.set_zero(result == 0);
            args.register.set_negative(result & (0b1u64 << 63) != 0);
            args.register.set_general(&reg1, result)?;
        }
        ADD_REG_W_NUM => {
            let reg1 = args.argument.parse_register()?;
            let num = args.argument.parse_u64()?;
            let n_reg1 = args.register.get_general(&reg1)?;
            let (result, overflow) = n_reg1.overflowing_add(num);
            args.register.set_carry(overflow);
            args.register.set_zero(result == 0);
            args.register.set_negative(result & (0b1u64 << 63) != 0);
            args.register.set_general(&reg1, result)?;
        }
        ADD_SP_W_NUM => {
            args.argument.parse_register()?;
            let num = args.argument.parse_u64()?;
            let (result, overflow) = args
                .register
                .get_sp()
                .get_raw()
                .overflowing_add(num as usize);
            args.register.set_carry(overflow);
            args.register.set_zero(result == 0);
            args.register
                .set_negative(result as u64 & (0b1u64 << 63) != 0);
            args.register.set_sp(Address::new(result as usize));
        }
        invalid_subop_code => {
            return Err(super::InstructionError::InvalidSubOpCode(
                ADD_OPCODE,
                invalid_subop_code,
            ));
        }
    }
    return Ok(());
}
