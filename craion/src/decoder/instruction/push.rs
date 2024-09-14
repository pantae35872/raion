use proc::instruction;

use crate::executor::registers::RegisterSizes;

use super::InstructionArgument;

#[instruction(PUSH_OPCODE)]
pub fn push(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    args.register.inc_ip(args.instruction_length);
    let reg = args.argument.parse_register()?;
    let value = args.register.get_general(&reg)?;
    let sp = args.register.dec_sp(reg.size().byte());
    match reg.size() {
        RegisterSizes::SizeU8 => {
            args.memory.mem_set(sp, value as u8)?;
        }
        RegisterSizes::SizeU16 => {
            args.memory.mem_sets(sp, &(value as u16).to_le_bytes())?;
        }
        RegisterSizes::SizeU32 => {
            args.memory.mem_sets(sp, &(value as u32).to_le_bytes())?;
        }
        RegisterSizes::SizeU64 => {
            args.memory.mem_sets(sp, &(value as u64).to_le_bytes())?;
        }
        RegisterSizes::SizeBool => unreachable!(),
    }
    return Ok(());
}
