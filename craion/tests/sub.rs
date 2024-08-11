use craion::{
    decoder::instruction::SUB_OPCODE,
    executor::{
        registers::{RegisterFile, Registers},
        Executor,
    },
    instruction_helper::InstructionHelper,
    memory::{argument_memory::ArgumentMemory, Memory},
};

#[test]
fn normal_sub() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(SUB_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 8).unwrap();
    register.set_general(&Registers::B64, 5).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::A64).unwrap(), 3);
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), false);
    assert_eq!(register.get_carry(), false);
}

#[test]
fn zero_sub() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(SUB_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 3).unwrap();
    register.set_general(&Registers::B64, 3).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::A64).unwrap(), 0);
    assert_eq!(register.get_zero(), true);
    assert_eq!(register.get_negative(), false);
    assert_eq!(register.get_carry(), false);
}
#[test]
fn negative_sub() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(SUB_OPCODE, &[4, 8])
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
    assert_eq!(
        register.get_general(&Registers::A64).unwrap(),
        18446744073709551614 // -2
    );
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), true);
    assert_eq!(register.get_carry(), false);
}
#[test]
fn carry_sub() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(SUB_OPCODE, &[4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 2).unwrap();
    register.set_general(&Registers::B64, 3).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::A64).unwrap(),
        18446744073709551615 // -1
    );
    assert_eq!(register.get_zero(), false);
    assert_eq!(register.get_negative(), true);
    assert_eq!(register.get_carry(), true);
}
