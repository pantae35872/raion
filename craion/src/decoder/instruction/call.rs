use proc::instruction;

use super::InstructionArgument;

#[instruction(CALL_OPCODE, "crate::decoder::instruction::call::call")]
pub fn call(_args: &mut InstructionArgument) -> Result<(), super::InstructionError> {
    //let procedure_hash = args.argument.parse_u64()?;
    //args.register.inc_ip(args.instruction_length);
    //args.ret_stack.push(args.register.get_ip());
    //let section = args
    //    .section_manager
    //    .get_section_hash(procedure_hash)
    //    .ok_or(InstructionError::InvalidSection(procedure_hash))?;

    //if section.section_type() == SectionType::Procedure {
    //    args.register.set_ip(section.mem_start());
    //} else {
    //    return Err(InstructionError::NotProcedureSection);
    //}
    return Ok(());
}
