use proc::instruction;

use crate::executor::registers::RegisterSizes;

use super::InstructionArgument;

#[instruction(POP_OPCODE)]
pub fn pop(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let reg = args.argument.parse_register()?;
    let sp = args.register.get_sp();
    match reg.size() {
        RegisterSizes::SizeU8 => {
            args.register
                .set_general(&reg, args.memory.mem_get(sp)? as u64)?;
        }
        RegisterSizes::SizeU16 => {
            args.register.set_general(
                &reg,
                u16::from_le_bytes(<[u8; 2]>::try_from(args.memory.mem_gets(sp, 2)?).unwrap())
                    .into(),
            )?;
        }
        RegisterSizes::SizeU32 => {
            args.register.set_general(
                &reg,
                u32::from_le_bytes(<[u8; 4]>::try_from(args.memory.mem_gets(sp, 4)?).unwrap())
                    .into(),
            )?;
        }
        RegisterSizes::SizeU64 => {
            args.register.set_general(
                &reg,
                u64::from_le_bytes(<[u8; 8]>::try_from(args.memory.mem_gets(sp, 8)?).unwrap()),
            )?;
        }
        RegisterSizes::SizeBool => unreachable!(),
    }
    args.register.inc_sp(reg.size().byte());
    return Ok(());
}
