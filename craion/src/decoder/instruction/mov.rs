use common::{
    constants::{
        MOV_ADD2SP, MOV_DEREF_REG2REG, MOV_NUM2REG, MOV_OPCODE, MOV_REG2MEM, MOV_REG2REG,
        MOV_REG2SP, MOV_SECTION_ADDR_2REG,
    },
    memory::buffer_reader::BufferReader,
};
use proc::instruction;

use crate::{executor::registers::RegisterSizes, memory::address::Address};

use super::{InstructionArgument, InstructionError};

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
                RegisterSizes::SizeBool => unreachable!(),
            }
        }
        MOV_SECTION_ADDR_2REG => {
            let register = args.argument.parse_register()?;
            let section_hash = args.argument.parse_u64()?;
            let value = args
                .section_manager
                .get_section_hash(section_hash)
                .ok_or(InstructionError::InvalidSection(section_hash))?;
            match register.size() {
                RegisterSizes::SizeU64 => {
                    args.register
                        .set_general(&register, value.mem_start().get_raw() as u64)?;
                }
                _ => {
                    return Err(InstructionError::AddressToRegisterError(
                        register.size().byte(),
                    ))
                }
            }
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
