use common::{constants::SUB_OPCODE, register::RegisterType};
use craion::{executor::Executor, instruction_helper::InstructionHelper};

#[test]
fn normal_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(SUB_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 8)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 5)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        3
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), false);
    assert_eq!(executor.registers().get_carry(), false);
}

#[test]
fn zero_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(SUB_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 3)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 3)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        0
    );
    assert_eq!(executor.registers().get_zero(), true);
    assert_eq!(executor.registers().get_negative(), false);
    assert_eq!(executor.registers().get_carry(), false);
}
#[test]
fn negative_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(SUB_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 18446744073709551615)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 1)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        18446744073709551614 // -2
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), true);
    assert_eq!(executor.registers().get_carry(), false);
}
#[test]
fn carry_sub() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(SUB_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 2)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 3)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        18446744073709551615 // -1
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), true);
    assert_eq!(executor.registers().get_carry(), true);
}
