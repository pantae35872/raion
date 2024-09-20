use common::{constants::ADD_OPCODE, register::RegisterType};
use craion::{executor::Executor, instruction_helper::InstructionHelper};

#[test]
fn normal_add() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(ADD_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 5)
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
        8
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), false);
    assert_eq!(executor.registers().get_carry(), false);
}
#[test]
fn carry_add() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(ADD_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 5)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 0xFFFFFFFFFFFFFFFF)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        4
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), false);
    assert_eq!(executor.registers().get_carry(), true);
}

#[test]
fn zero_add() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(ADD_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 0)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 0)
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
fn negative_add() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(ADD_OPCODE)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 1)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 0x7FFFFFFFFFFFFFFF)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        9223372036854775808 // -1
    );
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), true);
    assert_eq!(executor.registers().get_carry(), false);
}
