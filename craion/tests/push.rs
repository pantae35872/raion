mod instruction_helper;

use common::constants::PUSH_OPCODE;
use craion::{
    executor::{registers::Registers, Executor},
    memory::address::Address,
};
use instruction_helper::InstructionHelper;

#[test]
fn push_u64() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE, &[4])
        .unwrap()
        .encode(PUSH_OPCODE, &[8])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A64, 33)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B64, 687545)
        .unwrap();
    executor.registers().set_sp(Address::new(255));
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::B64).unwrap(),
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
        executor.registers().get_general(&Registers::A64).unwrap(),
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
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE, &[3])
        .unwrap()
        .encode(PUSH_OPCODE, &[7])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A32, 56138)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B32, 42487)
        .unwrap();
    executor.registers().set_sp(Address::new(255));
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::B32).unwrap(),
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
        executor.registers().get_general(&Registers::A32).unwrap(),
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
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE, &[2])
        .unwrap()
        .encode(PUSH_OPCODE, &[6])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A16, 2454)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B16, 180)
        .unwrap();
    executor.registers().set_sp(Address::new(255));
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::B16).unwrap(),
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
        executor.registers().get_general(&Registers::A16).unwrap(),
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
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE, &[1])
        .unwrap()
        .encode(PUSH_OPCODE, &[5])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A8, 24)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B8, 211)
        .unwrap();
    executor.registers().set_sp(Address::new(255));
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::B8).unwrap(),
        executor
            .memory_ref()
            .mem_get(executor.registers_ref().get_sp())
            .unwrap()
            .into()
    );
    assert_eq!(
        executor.registers().get_general(&Registers::A8).unwrap(),
        executor
            .memory_ref()
            .mem_get(executor.registers_ref().get_sp() + 1)
            .unwrap()
            .into()
    );
}
