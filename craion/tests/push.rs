mod instruction_helper;

use common::constants::PUSH_OPCODE;
use craion::{
    executor::{
        registers::{RegisterFile, Registers},
        Executor,
    },
    memory::{address::Address, argument_memory::ArgumentMemory, Memory},
};
use instruction_helper::InstructionHelper;

#[test]
fn push_u64() {
    let mut memory = Memory::new(256);
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[4])
        .unwrap()
        .encode(PUSH_OPCODE, &[8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 33).unwrap();
    register.set_general(&Registers::B64, 687545).unwrap();
    register.set_sp(Address::new(255));
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::B64).unwrap(),
        u64::from_le_bytes(
            <[u8; 8]>::try_from(memory.mem_gets(register.get_sp(), 8).unwrap()).unwrap()
        ),
    );
    assert_eq!(
        register.get_general(&Registers::A64).unwrap(),
        u64::from_le_bytes(
            <[u8; 8]>::try_from(memory.mem_gets(register.get_sp() + 8, 8).unwrap()).unwrap()
        ),
    );
}
#[test]
fn push_u32() {
    let mut memory = Memory::new(256);
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[3])
        .unwrap()
        .encode(PUSH_OPCODE, &[7])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A32, 56138).unwrap();
    register.set_general(&Registers::B32, 42487).unwrap();
    register.set_sp(Address::new(255));
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::B32).unwrap(),
        u32::from_le_bytes(
            <[u8; 4]>::try_from(memory.mem_gets(register.get_sp(), 4).unwrap()).unwrap()
        )
        .into(),
    );
    assert_eq!(
        register.get_general(&Registers::A32).unwrap(),
        u32::from_le_bytes(
            <[u8; 4]>::try_from(memory.mem_gets(register.get_sp() + 4, 4).unwrap()).unwrap()
        )
        .into(),
    );
}
#[test]
fn push_u16() {
    let mut memory = Memory::new(256);
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[2])
        .unwrap()
        .encode(PUSH_OPCODE, &[6])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A16, 2454).unwrap();
    register.set_general(&Registers::B16, 180).unwrap();
    register.set_sp(Address::new(255));
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::B16).unwrap(),
        u16::from_le_bytes(
            <[u8; 2]>::try_from(memory.mem_gets(register.get_sp(), 2).unwrap()).unwrap()
        )
        .into(),
    );
    assert_eq!(
        register.get_general(&Registers::A16).unwrap(),
        u16::from_le_bytes(
            <[u8; 2]>::try_from(memory.mem_gets(register.get_sp() + 2, 2).unwrap()).unwrap()
        )
        .into(),
    );
}

#[test]
fn push_u8() {
    let mut memory = Memory::new(256);
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[1])
        .unwrap()
        .encode(PUSH_OPCODE, &[5])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A8, 24).unwrap();
    register.set_general(&Registers::B8, 211).unwrap();
    register.set_sp(Address::new(255));
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::B8).unwrap(),
        memory.mem_get(register.get_sp()).unwrap().into()
    );
    assert_eq!(
        register.get_general(&Registers::A8).unwrap(),
        memory.mem_get(register.get_sp() + 1).unwrap().into()
    );
}
