mod instruction_helper;

use common::constants::CMP_OPCODE;
use craion::{
    executor::{
        registers::{RegisterFile, Registers},
        Executor,
    },
    memory::{argument_memory::ArgumentMemory, Memory},
};
use instruction_helper::InstructionHelper;

#[test]
fn carry_cmp() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(CMP_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 1).unwrap();
    register.set_general(&Registers::B64, 2).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), true);
    assert_eq!(register.get_carry(), true);
}

#[test]
fn zero_cmp() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(CMP_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 1).unwrap();
    register.set_general(&Registers::B64, 1).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_zero(), true);
    assert_eq!(register.get_negative(), false);
    assert_eq!(register.get_carry(), false);
}

#[test]
fn negative_cmp() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(CMP_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register
        .set_general(&Registers::A64, 18446744073709551615)
        .unwrap();
    register.set_general(&Registers::B64, 1).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), true);
    assert_eq!(register.get_carry(), false);
}
