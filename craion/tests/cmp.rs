use common::{constants::CMP_OPCODE, register::RegisterType};
use craion::{executor::Executor, instruction_helper::InstructionHelper};

#[test]
fn carry_cmp() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(CMP_OPCODE)
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
        .set_general(&RegisterType::B64, 2)
        .unwrap();
    executor.execute();
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), true);
    assert_eq!(executor.registers().get_carry(), true);
}

#[test]
fn zero_cmp() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(CMP_OPCODE)
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
        .set_general(&RegisterType::B64, 1)
        .unwrap();
    executor.execute();
    assert_eq!(executor.registers().get_zero(), true);
    assert_eq!(executor.registers().get_negative(), false);
    assert_eq!(executor.registers().get_carry(), false);
}

#[test]
fn negative_cmp() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(CMP_OPCODE)
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
    assert_eq!(executor.registers().get_zero(), false);
    assert_eq!(executor.registers().get_negative(), true);
    assert_eq!(executor.registers().get_carry(), false);
}
