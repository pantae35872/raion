use common::{
    constants::{
        MOV_ADD2SP, MOV_DEREF_REG2REG,
        MOV_DEREF_REG_WITH_ADD_OFFSET2DEREF_REG_WITH_ADD_OFFSET_WITH_SIZE,
        MOV_DEREF_REG_WITH_ADD_OFFSET2DEREF_REG_WITH_SUB_OFFSET_WITH_SIZE,
        MOV_DEREF_REG_WITH_SUB_OFFSET2DEREF_REG_WITH_ADD_OFFSET_WITH_SIZE,
        MOV_DEREF_REG_WITH_SUB_OFFSET2DEREF_REG_WITH_SUB_OFFSET_WITH_SIZE,
        MOV_DEREF_SP_WITH_ADD_OFFSET2DEREF_SP_WITH_ADD_OFFSET_WITH_SIZE,
        MOV_DEREF_SP_WITH_ADD_OFFSET2DEREF_SP_WITH_SUB_OFFSET_WITH_SIZE,
        MOV_DEREF_SP_WITH_ADD_OFFSET2REG,
        MOV_DEREF_SP_WITH_SUB_OFFSET2DEREF_SP_WITH_ADD_OFFSET_WITH_SIZE,
        MOV_DEREF_SP_WITH_SUB_OFFSET2DEREF_SP_WITH_SUB_OFFSET_WITH_SIZE,
        MOV_DEREF_SP_WITH_SUB_OFFSET2REG, MOV_NUM2DEREF_REG, MOV_NUM2DEREF_REG_WITH_ADD_OFFSET,
        MOV_NUM2DEREF_REG_WITH_SUB_OFFSET, MOV_NUM2DEREF_SP_WITH_ADD_OFFSET,
        MOV_NUM2DEREF_SP_WITH_SUB_OFFSET, MOV_NUM2REG, MOV_OPCODE, MOV_REG2DEREF_REG,
        MOV_REG2DEREF_REG_WITH_ADD_OFFSET, MOV_REG2DEREF_REG_WITH_SUB_OFFSET,
        MOV_REG2DEREF_SP_WITH_ADD_OFFSET, MOV_REG2DEREF_SP_WITH_SUB_OFFSET, MOV_REG2REG,
        MOV_REG2SP, MOV_SECTION_ADDR2DEREF_REG_WITH_ADD_OFFSET,
        MOV_SECTION_ADDR2DEREF_SP_WITH_ADD_OFFSET, MOV_SECTION_ADDR2DEREF_SP_WITH_SUB_OFFSET,
        MOV_SECTION_ADDR_2REG,
    },
    memory::buffer_reader::BufferReader,
    register::RegisterSizes,
};
use proc::instruction;

use crate::memory::address::Address;

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
        MOV_NUM2DEREF_REG => {
            let reg = args.argument.parse_register()?;
            let num = args.argument.parse_u64()?;
            let address = Address::new(args.register.get_general(&reg)? as usize);
            args.memory.mem_sets(address, &num.to_le_bytes())?;
        }
        MOV_NUM2DEREF_REG_WITH_ADD_OFFSET => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let num = args.argument.parse_u64()?;
            let address = Address::new(args.register.get_general(&reg)? as usize);
            args.memory
                .mem_sets(address + offset as usize, &num.to_le_bytes())?;
        }
        MOV_NUM2DEREF_REG_WITH_SUB_OFFSET => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let num = args.argument.parse_u64()?;
            args.memory
                .mem_sets(args.register.get_sp() - offset as usize, &num.to_le_bytes())?;
        }
        MOV_NUM2DEREF_SP_WITH_ADD_OFFSET => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let num = args.argument.parse_u64()?;
            args.memory
                .mem_sets(args.register.get_sp() + offset as usize, &num.to_le_bytes())?;
        }
        MOV_NUM2DEREF_SP_WITH_SUB_OFFSET => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let num = args.argument.parse_u64()?;
            args.memory
                .mem_sets(args.register.get_sp() - offset as usize, &num.to_le_bytes())?;
        }
        MOV_DEREF_SP_WITH_ADD_OFFSET2DEREF_SP_WITH_ADD_OFFSET_WITH_SIZE => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let data = args
                .memory
                .mem_gets(args.register.get_sp() + offset2 as usize, size as usize)?
                .to_vec();
            args.memory
                .mem_sets(args.register.get_sp() + offset as usize, &data)?;
        }
        MOV_DEREF_SP_WITH_ADD_OFFSET2DEREF_SP_WITH_SUB_OFFSET_WITH_SIZE => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let data = args
                .memory
                .mem_gets(args.register.get_sp() - offset2 as usize, size as usize)?
                .to_vec();
            args.memory
                .mem_sets(args.register.get_sp() + offset as usize, &data)?;
        }
        MOV_DEREF_SP_WITH_SUB_OFFSET2DEREF_SP_WITH_ADD_OFFSET_WITH_SIZE => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let data = args
                .memory
                .mem_gets(args.register.get_sp() + offset2 as usize, size as usize)?
                .to_vec();
            args.memory
                .mem_sets(args.register.get_sp() - offset as usize, &data)?;
        }
        MOV_DEREF_SP_WITH_SUB_OFFSET2DEREF_SP_WITH_SUB_OFFSET_WITH_SIZE => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let data = args
                .memory
                .mem_gets(args.register.get_sp() - offset2 as usize, size as usize)?
                .to_vec();
            args.memory
                .mem_sets(args.register.get_sp() - offset as usize, &data)?;
        }
        MOV_DEREF_REG_WITH_ADD_OFFSET2DEREF_REG_WITH_ADD_OFFSET_WITH_SIZE => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let reg2 = args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let reg_addr = Address::new(args.register.get_general(&reg)? as usize);
            let reg2_addr = Address::new(args.register.get_general(&reg2)? as usize);
            let data = args
                .memory
                .mem_gets(reg2_addr + offset2 as usize, size as usize)?
                .to_vec();
            args.memory.mem_sets(reg_addr + offset as usize, &data)?;
        }
        MOV_DEREF_REG_WITH_ADD_OFFSET2DEREF_REG_WITH_SUB_OFFSET_WITH_SIZE => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let reg2 = args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let reg_addr = Address::new(args.register.get_general(&reg)? as usize);
            let reg2_addr = Address::new(args.register.get_general(&reg2)? as usize);
            let data = args
                .memory
                .mem_gets(reg2_addr - offset2 as usize, size as usize)?
                .to_vec();
            args.memory.mem_sets(reg_addr + offset as usize, &data)?;
        }
        MOV_DEREF_REG_WITH_SUB_OFFSET2DEREF_REG_WITH_ADD_OFFSET_WITH_SIZE => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let reg2 = args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let reg_addr = Address::new(args.register.get_general(&reg)? as usize);
            let reg2_addr = Address::new(args.register.get_general(&reg2)? as usize);
            let data = args
                .memory
                .mem_gets(reg2_addr + offset2 as usize, size as usize)?
                .to_vec();
            args.memory.mem_sets(reg_addr - offset as usize, &data)?;
        }
        MOV_DEREF_REG_WITH_SUB_OFFSET2DEREF_REG_WITH_SUB_OFFSET_WITH_SIZE => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let reg2 = args.argument.parse_register()?;
            let offset2 = args.argument.parse_u32()?;
            let size = args.argument.parse_u64()?;
            let reg_addr = Address::new(args.register.get_general(&reg)? as usize);
            let reg2_addr = Address::new(args.register.get_general(&reg2)? as usize);
            let data = args
                .memory
                .mem_gets(reg2_addr - offset2 as usize, size as usize)?
                .to_vec();
            args.memory.mem_sets(reg_addr - offset as usize, &data)?;
        }
        MOV_REG2DEREF_REG_WITH_ADD_OFFSET => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let source_reg = args.argument.parse_register()?;
            let n_source_reg = args.register.get_general(&source_reg)?;
            let reg_addr = Address::new(args.register.get_general(&reg)? as usize);
            match source_reg.size() {
                RegisterSizes::SizeU8 => {
                    args.memory.mem_sets(
                        reg_addr + offset as usize,
                        &(n_source_reg as u8).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU16 => {
                    args.memory.mem_sets(
                        reg_addr + offset as usize,
                        &(n_source_reg as u16).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU32 => {
                    args.memory.mem_sets(
                        reg_addr + offset as usize,
                        &(n_source_reg as u32).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU64 => {
                    args.memory
                        .mem_sets(reg_addr + offset as usize, &n_source_reg.to_le_bytes())?;
                }
            }
        }
        MOV_REG2DEREF_REG_WITH_SUB_OFFSET => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let source_reg = args.argument.parse_register()?;
            let n_source_reg = args.register.get_general(&source_reg)?;
            let reg_addr = Address::new(args.register.get_general(&reg)? as usize);
            match source_reg.size() {
                RegisterSizes::SizeU8 => {
                    args.memory.mem_sets(
                        reg_addr - offset as usize,
                        &(n_source_reg as u8).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU16 => {
                    args.memory.mem_sets(
                        reg_addr - offset as usize,
                        &(n_source_reg as u16).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU32 => {
                    args.memory.mem_sets(
                        reg_addr - offset as usize,
                        &(n_source_reg as u32).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU64 => {
                    args.memory
                        .mem_sets(reg_addr - offset as usize, &n_source_reg.to_le_bytes())?;
                }
            }
        }
        MOV_REG2DEREF_SP_WITH_SUB_OFFSET => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let source_reg = args.argument.parse_register()?;
            let n_source_reg = args.register.get_general(&source_reg)?;
            match source_reg.size() {
                RegisterSizes::SizeU8 => {
                    args.memory.mem_sets(
                        args.register.get_sp() + offset as usize,
                        &(n_source_reg as u8).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU16 => {
                    args.memory.mem_sets(
                        args.register.get_sp() + offset as usize,
                        &(n_source_reg as u16).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU32 => {
                    args.memory.mem_sets(
                        args.register.get_sp() + offset as usize,
                        &(n_source_reg as u32).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU64 => {
                    args.memory.mem_sets(
                        args.register.get_sp() + offset as usize,
                        &n_source_reg.to_le_bytes(),
                    )?;
                }
            }
        }
        MOV_REG2DEREF_SP_WITH_ADD_OFFSET => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let source_reg = args.argument.parse_register()?;
            let n_source_reg = args.register.get_general(&source_reg)?;
            match source_reg.size() {
                RegisterSizes::SizeU8 => {
                    args.memory.mem_sets(
                        args.register.get_sp() - offset as usize,
                        &(n_source_reg as u8).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU16 => {
                    args.memory.mem_sets(
                        args.register.get_sp() - offset as usize,
                        &(n_source_reg as u16).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU32 => {
                    args.memory.mem_sets(
                        args.register.get_sp() - offset as usize,
                        &(n_source_reg as u32).to_le_bytes(),
                    )?;
                }
                RegisterSizes::SizeU64 => {
                    args.memory.mem_sets(
                        args.register.get_sp() - offset as usize,
                        &n_source_reg.to_le_bytes(),
                    )?;
                }
            }
        }
        MOV_SECTION_ADDR2DEREF_SP_WITH_ADD_OFFSET => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let section_hash = args.argument.parse_u64()?;
            let value = args
                .section_manager
                .get_section_hash(section_hash)
                .ok_or(InstructionError::InvalidSection(section_hash))?;
            args.memory.mem_sets(
                args.register.get_sp() + offset as usize,
                &value.mem_start().get_raw().to_le_bytes(),
            )?;
        }
        MOV_SECTION_ADDR2DEREF_SP_WITH_SUB_OFFSET => {
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let section_hash = args.argument.parse_u64()?;
            let value = args
                .section_manager
                .get_section_hash(section_hash)
                .ok_or(InstructionError::InvalidSection(section_hash))?;
            args.memory.mem_sets(
                args.register.get_sp() - offset as usize,
                &value.mem_start().get_raw().to_le_bytes(),
            )?;
        }
        MOV_SECTION_ADDR2DEREF_REG_WITH_ADD_OFFSET => {
            let reg = args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            let section_hash = args.argument.parse_u64()?;
            let value = args
                .section_manager
                .get_section_hash(section_hash)
                .ok_or(InstructionError::InvalidSection(section_hash))?;
            let reg_addr = Address::new(args.register.get_general(&reg)? as usize);
            args.memory.mem_sets(
                reg_addr - offset as usize,
                &value.mem_start().get_raw().to_le_bytes(),
            )?;
        }
        MOV_DEREF_SP_WITH_ADD_OFFSET2REG => {
            let reg = args.argument.parse_register()?;
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            match reg.size() {
                RegisterSizes::SizeU64 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() + offset as usize, 8)?;
                    let data = BufferReader::new(data).read_u64().unwrap();
                    args.register.set_general(&reg, data)?;
                }
                RegisterSizes::SizeU32 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() + offset as usize, 4)?;
                    let data = BufferReader::new(data).read_u32().unwrap();
                    args.register.set_general(&reg, data.into())?;
                }
                RegisterSizes::SizeU16 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() + offset as usize, 2)?;
                    let data = BufferReader::new(data).read_u16().unwrap();
                    args.register.set_general(&reg, data.into())?;
                }
                RegisterSizes::SizeU8 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() + offset as usize, 1)?;
                    let data = BufferReader::new(data).read_u8().unwrap();
                    args.register.set_general(&reg, data.into())?;
                }
            }
        }
        MOV_DEREF_SP_WITH_SUB_OFFSET2REG => {
            let reg = args.argument.parse_register()?;
            args.argument.parse_register()?;
            let offset = args.argument.parse_u32()?;
            match reg.size() {
                RegisterSizes::SizeU64 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() - offset as usize, 8)?;
                    let data = BufferReader::new(data).read_u64().unwrap();
                    args.register.set_general(&reg, data)?;
                }
                RegisterSizes::SizeU32 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() - offset as usize, 4)?;
                    let data = BufferReader::new(data).read_u32().unwrap();
                    args.register.set_general(&reg, data.into())?;
                }
                RegisterSizes::SizeU16 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() - offset as usize, 2)?;
                    let data = BufferReader::new(data).read_u16().unwrap();
                    args.register.set_general(&reg, data.into())?;
                }
                RegisterSizes::SizeU8 => {
                    let data = args
                        .memory
                        .mem_gets(args.register.get_sp() - offset as usize, 1)?;
                    let data = BufferReader::new(data).read_u8().unwrap();
                    args.register.set_general(&reg, data.into())?;
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
