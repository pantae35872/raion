use crate::executor::registers::Register;

pub trait Instruction {
    fn execute(register: &mut Register);
}
