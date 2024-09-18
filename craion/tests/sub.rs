mod instruction_helper;

use common::constants::SUB_OPCODE;
use craion::executor::{registers::Registers, Executor};
use instruction_helper::InstructionHelper;

#[test]
fn normal_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(SUB_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A64, 8)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B64, 5)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::A64).unwrap(),
        3
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), false);
    assert_eq!(executor.registers().get_carry(), false);
}

#[test]
fn zero_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(SUB_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A64, 3)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B64, 3)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::A64).unwrap(),
        0
    );
    assert_eq!(executor.registers().get_zero(), true);
    assert_eq!(executor.registers().get_negative(), false);
    assert_eq!(executor.registers().get_carry(), false);
}
#[test]
fn negative_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(SUB_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A64, 18446744073709551615)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B64, 1)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::A64).unwrap(),
        18446744073709551614 // -2
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), true);
    assert_eq!(executor.registers().get_carry(), false);
}
#[test]
fn carry_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(SUB_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A64, 2)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B64, 3)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::A64).unwrap(),
        18446744073709551615 // -1
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), true);
    assert_eq!(executor.registers().get_carry(), true);
}
