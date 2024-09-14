use common::constants::ADD_OPCODE;
use craion::{
    executor::{
        registers::{RegisterFile, Registers},
        Executor,
    },
    instruction_helper::InstructionHelper,
    memory::{argument_memory::ArgumentMemory, Memory},
};

#[test]
fn normal_add() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(ADD_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 5).unwrap();
    register.set_general(&Registers::B64, 3).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::A64).unwrap(), 8);
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), false);
    assert_eq!(register.get_carry(), false);
}
#[test]
fn carry_add() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(ADD_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 5).unwrap();
    register
        .set_general(&Registers::B64, 0xFFFFFFFFFFFFFFFF)
        .unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::A64).unwrap(), 4);
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), false);
    assert_eq!(register.get_carry(), true);
}

#[test]
fn zero_add() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(ADD_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 0).unwrap();
    register.set_general(&Registers::B64, 0).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::A64).unwrap(), 0);
    assert_eq!(register.get_zero(), true);
    assert_eq!(register.get_negative(), false);
    assert_eq!(register.get_carry(), false);
}
#[test]
fn negative_add() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(ADD_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 1).unwrap();
    register
        .set_general(&Registers::B64, 0x7FFFFFFFFFFFFFFF)
        .unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::A64).unwrap(),
        9223372036854775808 // -1
    );
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), true);
    assert_eq!(register.get_carry(), false);
}
