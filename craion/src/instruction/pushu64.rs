use proc::instruction;

use crate::{
    decoder::argument::Argument,
    executor::{
        objects::{Object, Primitive},
        ExecutorState,
    },
    section_manager::LoadedType,
};

#[instruction(PUSHU64_OPCODE, "crate::instruction::pushu64::pushu64")]
pub fn pushu64(
    state: &mut ExecutorState,
    argument: &mut Argument,
    instruction_length: usize,
) -> Result<(), super::InstructionError> {
    state.program_state.inc_ip(instruction_length);
    let value = argument.parse_u64()?;
    let mut object = Object::new(LoadedType::U64);
    object.set_primtive(Primitive::U64(value));
    state.operand_stack.push(object);
    return Ok(());
}
