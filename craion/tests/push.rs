use common::{constants::PUSH_OPCODE, register::RegisterType};
use craion::{executor::Executor, instruction_helper::InstructionHelper, memory::address::Address};

#[test]
fn push_u64() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A64)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 33)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 687545)
        .unwrap();
    executor.registers().set_sp(Address::new(0xFFFE));
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::B64)
            .unwrap(),
        u64::from_le_bytes(
            <[u8; 8]>::try_from(
                executor
                    .memory_ref()
                    .mem_gets(executor.registers_ref().get_sp(), 8)
                    .unwrap()
            )
            .unwrap()
        ),
    );
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        u64::from_le_bytes(
            <[u8; 8]>::try_from(
                executor
                    .memory_ref()
                    .mem_gets(executor.registers_ref().get_sp() + 8, 8)
                    .unwrap()
            )
            .unwrap()
        ),
    );
}
#[test]
fn push_u32() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A32)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B32)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A32, 56138)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B32, 42487)
        .unwrap();
    executor.registers().set_sp(Address::new(255));
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::B32)
            .unwrap(),
        u32::from_le_bytes(
            <[u8; 4]>::try_from(
                executor
                    .memory_ref()
                    .mem_gets(executor.registers_ref().get_sp(), 4)
                    .unwrap()
            )
            .unwrap()
        )
        .into(),
    );
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A32)
            .unwrap(),
        u32::from_le_bytes(
            <[u8; 4]>::try_from(
                executor
                    .memory_ref()
                    .mem_gets(executor.registers_ref().get_sp() + 4, 4)
                    .unwrap()
            )
            .unwrap()
        )
        .into(),
    );
}
#[test]
fn push_u16() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A16)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B16)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A16, 2454)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B16, 180)
        .unwrap();
    executor.registers().set_sp(Address::new(255));
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::B16)
            .unwrap(),
        u16::from_le_bytes(
            <[u8; 2]>::try_from(
                executor
                    .memory_ref()
                    .mem_gets(executor.registers_ref().get_sp(), 2)
                    .unwrap()
            )
            .unwrap()
        )
        .into(),
    );
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A16)
            .unwrap(),
        u16::from_le_bytes(
            <[u8; 2]>::try_from(
                executor
                    .memory_ref()
                    .mem_gets(executor.registers_ref().get_sp() + 2, 2)
                    .unwrap()
            )
            .unwrap()
        )
        .into(),
    );
}

#[test]
fn push_u8() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A8)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B8)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A8, 24)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B8, 211)
        .unwrap();
    executor.registers().set_sp(Address::new(255));
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&RegisterType::B8).unwrap(),
        executor
            .memory_ref()
            .mem_get(executor.registers_ref().get_sp())
            .unwrap()
            .into()
    );
    assert_eq!(
        executor.registers().get_general(&RegisterType::A8).unwrap(),
        executor
            .memory_ref()
            .mem_get(executor.registers_ref().get_sp() + 1)
            .unwrap()
            .into()
    );
}
