use common::constants::{
    MOV_ADD2SP, MOV_NUM2REG, MOV_OPCODE, MOV_REG2MEM, MOV_REG2REG, MOV_REG2SP,
};
use proc::instruction;

use crate::{executor::registers::RegisterSizes, memory::address::Address};

use super::InstructionArgument;

#[instruction(MOV_OPCODE)]
pub fn mov(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    match args.argument.parse_u8()? {
        MOV_REG2REG => {
            args.register.set_general(
                &args.argument.parse_register()?,
                args.register
                    .get_general(&args.argument.parse_register()?)?,
            )?;
        }
        MOV_REG2MEM => {
            let address = args.argument.parse_address()?;
            let register = args.argument.parse_register()?;
            let value = args.register.get_general(&register)?;
            match register.size() {
                RegisterSizes::SizeU8 => {
                    args.memory.mem_set(address, value as u8)?;
                }
                RegisterSizes::SizeU16 => {
                    args.memory
                        .mem_sets(address, &(value as u16).to_le_bytes())?;
                }
                RegisterSizes::SizeU32 => {
                    args.memory
                        .mem_sets(address, &(value as u32).to_le_bytes())?;
                }
                RegisterSizes::SizeU64 => {
                    args.memory
                        .mem_sets(address, &(value as u64).to_le_bytes())?;
                }
                RegisterSizes::SizeBool => unreachable!(),
            }
        }
        MOV_NUM2REG => {
            let reg = args.argument.parse_register()?;
            args.register.set_general(
                &reg,
                match reg.size() {
                    RegisterSizes::SizeU8 => args.argument.parse_u8()?.into(),
                    RegisterSizes::SizeU16 => args.argument.parse_u16()?.into(),
                    RegisterSizes::SizeU32 => args.argument.parse_u32()?.into(),
                    RegisterSizes::SizeU64 => args.argument.parse_u64()?.into(),
                    RegisterSizes::SizeBool => 0,
                },
            )?;
        }
        MOV_ADD2SP => {
            args.register.set_sp(args.argument.parse_address()?);
        }
        MOV_REG2SP => {
            args.register.set_sp(Address::new(
                args.register
                    .get_general(&args.argument.parse_register()?)? as usize,
            ));
        }
        invalid_subop_code => {
            return Err(super::InstructionError::InvalidSubOpCode(
                MOV_OPCODE,
                invalid_subop_code,
            ))
        }
    };
    return Ok(());
}
