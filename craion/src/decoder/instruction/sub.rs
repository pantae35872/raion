use common::constants::{SUB_OPCODE, SUB_REG_W_NUM, SUB_REG_W_REG, SUB_SP_W_NUM};
use proc::instruction;

use crate::memory::address::Address;

use super::InstructionArgument;

#[instruction(SUB_OPCODE, "crate::decoder::instruction::sub::sub")]
pub fn sub(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);

    match args.argument.parse_u8()? {
        SUB_REG_W_REG => {
            let reg1 = args.argument.parse_register()?;
            let reg2 = args.argument.parse_register()?;
            let n_reg1 = args.register.get_general(&reg1)?;
            let n_reg2 = args.register.get_general(&reg2)?;
            let (result, overflow) = n_reg1.overflowing_sub(n_reg2);
            args.register.set_carry(overflow);
            args.register.set_zero(result == 0);
            args.register.set_negative(result & (0b1u64 << 63) != 0);
            args.register.set_general(&reg1, result)?;
        }
        SUB_REG_W_NUM => {
            let reg1 = args.argument.parse_register()?;
            let num = args.argument.parse_u64()?;
            let n_reg1 = args.register.get_general(&reg1)?;
            let (result, overflow) = n_reg1.overflowing_sub(num);
            args.register.set_carry(overflow);
            args.register.set_zero(result == 0);
            args.register.set_negative(result & (0b1u64 << 63) != 0);
            args.register.set_general(&reg1, result)?;
        }
        SUB_SP_W_NUM => {
            args.argument.parse_register()?;
            let num = args.argument.parse_u64()?;
            let (result, overflow) = args
                .register
                .get_sp()
                .get_raw()
                .overflowing_sub(num as usize);
            args.register.set_carry(overflow);
            args.register.set_zero(result == 0);
            args.register
                .set_negative(result as u64 & (0b1u64 << 63) != 0);
            args.register.set_sp(Address::new(result as usize));
        }
        invalid_subop_code => {
            return Err(super::InstructionError::InvalidSubOpCode(
                SUB_OPCODE,
                invalid_subop_code,
            ));
        }
    }
    return Ok(());
}
