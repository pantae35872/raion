use craion::{
    decoder::instruction::{
        mov::{MOV_REG2MEM, MOV_REG2REG},
        MOV_OPCODE,
    },
    executor::{
        registers::{RegisterFile, Registers},
        Executor,
    },
    instruction_helper::InstructionHelper,
    memory::{address::Address, argument_memory::ArgumentMemory, Memory},
};

#[test]
fn reg2reg() {
    let mut memory = Memory::new(64);
    InstructionHelper::new(&mut memory)
        .encode(MOV_OPCODE, &[MOV_REG2REG, 4, 8])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 32).unwrap();
    register.set_general(&Registers::B64, 64).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::A64).unwrap(),
        register.get_general(&Registers::B64).unwrap(),
    );
}

#[test]
fn reg2mem_u8() {
    let mut memory = Memory::new(64);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(63).get_raw().to_le_bytes());
    arg.push(1);
    InstructionHelper::new(&mut memory)
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A8, 32).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::A8).unwrap().to_le_bytes()[0],
        memory.mem_get(Address::new(63)).unwrap()
    );
}

#[test]
fn reg2mem_u16() {
    let mut memory = Memory::new(64);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(61).get_raw().to_le_bytes());
    arg.push(2);
    InstructionHelper::new(&mut memory)
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A16, 32).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        &register.get_general(&Registers::A16).unwrap().to_le_bytes()[0..2],
        memory.mem_gets(Address::new(61), 2).unwrap()
    );
}

#[test]
fn reg2mem_u32() {
    let mut memory = Memory::new(64);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(59).get_raw().to_le_bytes());
    arg.push(3);
    InstructionHelper::new(&mut memory)
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A32, 32).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        &register.get_general(&Registers::A32).unwrap().to_le_bytes()[0..4],
        memory.mem_gets(Address::new(59), 4).unwrap()
    );
}

#[test]
fn reg2mem_u64() {
    let mut memory = Memory::new(64);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(55).get_raw().to_le_bytes());
    arg.push(4);
    InstructionHelper::new(&mut memory)
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A64, 32).unwrap();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(
        register.get_general(&Registers::A64).unwrap().to_le_bytes(),
        memory.mem_gets(Address::new(55), 8).unwrap()
    );
}
