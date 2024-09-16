use common::sin::sections::SectionType;
use proc::instruction;

use super::{InstructionArgument, InstructionError};

#[instruction(CALL_OPCODE)]
pub fn call(args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    let function_hash = args.argument.parse_u64()?;
    args.register.inc_ip(args.instruction_length);
    args.ret_stack.push(args.register.get_ip());
    let section = args
        .section_manager
        .get_section_hash(function_hash)
        .ok_or(InstructionError::InvalidSection(function_hash))?;

    if section.section_type() == SectionType::Function {
        args.register.set_ip(section.mem_start());
    } else {
        return Err(InstructionError::NotFunctionSection);
    }
    return Ok(());
}
