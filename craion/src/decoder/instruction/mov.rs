use common::{
    constants::{
        MOV_ADD2SP, MOV_DEREF_REG2REG, MOV_DEREF_REG_WITH_OFFSET2REG, MOV_NUM2DEREF_REG,
        MOV_NUM2DEREF_REG_WITH_OFFSET, MOV_NUM2REG, MOV_OPCODE, MOV_REG2DEREF_REG,
        MOV_REG2DEREF_REG_WITH_OFFSET, MOV_REG2REG, MOV_REG2SP,
        MOV_SECTION_ADDR2DEREF_REG_WITH_OFFSET, MOV_SECTION_ADDR_2REG,
    },
    memory::buffer_reader::BufferReader,
    register::{RegisterSizes, RegisterType},
};
use proc::instruction;

use crate::memory::address::Address;

use super::{InstructionArgument, InstructionError};

#[instruction(MOV_OPCODE, "crate::decoder::instruction::mov::mov")]
pub fn mov(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    match args.argument.parse_u8()? {
        MOV_REG2REG => {
            let dst = args.argument.parse_register()?;
            args.register.reset_group(&dst.group());
            args.register.set_general(
                &dst,
                args.register
                    .get_general(&args.argument.parse_register()?)?,
            )?;
        }
        MOV_REG2DEREF_REG => {
            let register = args.argument.parse_register()?;
            let address = args.argument.parse_register()?;
            let address = Address::new(args.register.get_general(&address)? as usize);
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
            }
        }
        MOV_NUM2REG => {
            let reg = args.argument.parse_register()?;
            args.register.reset_group(&reg.group());
            args.register.set_general(
                &reg,
                match reg.size() {
                    RegisterSizes::SizeU8 => args.argument.parse_u8()?.into(),
                    RegisterSizes::SizeU16 => args.argument.parse_u16()?.into(),
                    RegisterSizes::SizeU32 => args.argument.parse_u32()?.into(),
                    RegisterSizes::SizeU64 => args.argument.parse_u64()?.into(),
                },
            )?;
        }
        MOV_ADD2SP => {
            args.argument.parse_register()?;
            args.register.set_sp(args.argument.parse_address()?);
        }
        MOV_REG2SP => {
            args.argument.parse_register()?;
            args.register.set_sp(Address::new(
                args.register
                    .get_general(&args.argument.parse_register()?)? as usize,
            ));
        }
        MOV_DEREF_REG2REG => {
            let register = args.argument.parse_register()?;
            let address = args.argument.parse_register()?;
            let address = Address::new(args.register.get_general(&address)? as usize);
            match register.size() {
                RegisterSizes::SizeU8 => {
                    let data = args.memory.mem_get(address)?;
                    args.register.set_general(&register, data.into())?;
                }
                RegisterSizes::SizeU16 => {
                    let data = args.memory.mem_gets(address, 2)?;
                    let data = BufferReader::new(data).read_u16().unwrap();
                    args.register.set_general(&register, data.into())?;
                }
                RegisterSizes::SizeU32 => {
                    let data = args.memory.mem_gets(address, 4)?;
                    let data = BufferReader::new(data).read_u32().unwrap();
                    args.register.set_general(&register, data.into())?;
                }
                RegisterSizes::SizeU64 => {
                    let data = args.memory.mem_gets(address, 8)?;
                    let data = BufferReader::new(data).read_u64().unwrap();
                    args.register.set_general(&register, data)?;
                }
            }
        }
        MOV_SECTION_ADDR_2REG => {
            let register = args.argument.parse_register()?;
            let section_start = args.parse_section()?.mem_start();
            match register.size() {
                RegisterSizes::SizeU64 => {
                    args.register
                        .set_general(&register, section_start.get_raw() as u64)?;
                }
                _ => {
                    return Err(InstructionError::AddressToRegisterError(
                        register.size().byte(),
                    ))
                }
            }
        }
        MOV_NUM2DEREF_REG => {
            let reg = args.argument.parse_register()?;
            let num = args.argument.parse_u64()?;
            let address = Address::new(args.register.get_general(&reg)? as usize);
            args.memory.mem_sets(address, &num.to_le_bytes())?;
        }
        MOV_NUM2DEREF_REG_WITH_OFFSET => {
            args.deref_offset_set(|args| Ok((args.argument.parse_u64()?.to_le_bytes(), 8)))?;
        }
        MOV_REG2DEREF_REG_WITH_OFFSET => {
            args.deref_offset_set(|args| {
                let reg = args.argument.parse_register()?;
                let value = match reg {
                    RegisterType::Sp => args.register.get_sp().get_raw() as u64,
                    _ => args.register.get_general(&reg)?,
                };
                match reg.size() {
                    RegisterSizes::SizeU64 => Ok((value.to_le_bytes(), 8)),
                    RegisterSizes::SizeU32 => Ok((value.to_le_bytes(), 4)),
                    RegisterSizes::SizeU16 => Ok((value.to_le_bytes(), 2)),
                    RegisterSizes::SizeU8 => Ok((value.to_le_bytes(), 1)),
                }
            })?;
        }
        MOV_DEREF_REG_WITH_OFFSET2REG => {
            let reg = args.argument.parse_register()?;
            let value = match reg.size() {
                RegisterSizes::SizeU64 => u64::from_le_bytes(args.deref_offset_get()?),
                RegisterSizes::SizeU32 => u32::from_le_bytes(args.deref_offset_get()?).into(),
                RegisterSizes::SizeU16 => u16::from_le_bytes(args.deref_offset_get()?).into(),
                RegisterSizes::SizeU8 => u8::from_le_bytes(args.deref_offset_get()?).into(),
            };
            args.register.set_general(&reg, value)?;
        }
        MOV_SECTION_ADDR2DEREF_REG_WITH_OFFSET => {
            args.deref_offset_set(|args| {
                Ok((args.parse_section()?.mem_start().get_raw().to_le_bytes(), 8))
            })?;
        }
        invalid_subop_code => {
            return Err(super::InstructionError::InvalidSubOpCode(
                MOV_OPCODE,
                invalid_subop_code,
            ));
        }
    };
    return Ok(());
}
