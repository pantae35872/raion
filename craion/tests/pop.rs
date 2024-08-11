use craion::{
    decoder::instruction::{mov::MOV_NUM2REG, MOV_OPCODE, POP_OPCODE, PUSH_OPCODE},
    executor::{
        registers::{RegisterFile, Registers},
        Executor,
    },
    instruction_helper::InstructionHelper,
    memory::{address::Address, argument_memory::ArgumentMemory, Memory},
};

#[test]
fn pop_u64() {
    let mut memory = Memory::new(256);
    let mut arg = [MOV_NUM2REG, 4].to_vec();
    arg.extend_from_slice(&610414u64.to_le_bytes());
    let mut arg2 = [MOV_NUM2REG, 8].to_vec();
    arg2.extend_from_slice(&45121u64.to_le_bytes());
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[4])
        .unwrap()
        .encode(PUSH_OPCODE, &[8])
        .unwrap()
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .encode(MOV_OPCODE, &arg2)
        .unwrap()
        .encode(POP_OPCODE, &[8])
        .unwrap()
        .encode(POP_OPCODE, &[4])
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
    assert_eq!(register.get_general(&Registers::B64).unwrap(), 687545);
    assert_eq!(register.get_general(&Registers::A64).unwrap(), 33);
}

#[test]
fn pop_u32() {
    let mut memory = Memory::new(256);
    let mut arg = [MOV_NUM2REG, 3].to_vec();
    arg.extend_from_slice(&642u32.to_le_bytes());
    let mut arg2 = [MOV_NUM2REG, 7].to_vec();
    arg2.extend_from_slice(&4454u32.to_le_bytes());
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[3])
        .unwrap()
        .encode(PUSH_OPCODE, &[7])
        .unwrap()
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .encode(MOV_OPCODE, &arg2)
        .unwrap()
        .encode(POP_OPCODE, &[7])
        .unwrap()
        .encode(POP_OPCODE, &[3])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A32, 1211).unwrap();
    register.set_general(&Registers::B32, 2154).unwrap();
    register.set_sp(Address::new(255));
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::B32).unwrap(), 2154);
    assert_eq!(register.get_general(&Registers::A32).unwrap(), 1211);
}
#[test]
fn pop_u16() {
    let mut memory = Memory::new(256);
    let mut arg = [MOV_NUM2REG, 2].to_vec();
    arg.extend_from_slice(&642u16.to_le_bytes());
    let mut arg2 = [MOV_NUM2REG, 6].to_vec();
    arg2.extend_from_slice(&4454u16.to_le_bytes());
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[2])
        .unwrap()
        .encode(PUSH_OPCODE, &[6])
        .unwrap()
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .encode(MOV_OPCODE, &arg2)
        .unwrap()
        .encode(POP_OPCODE, &[6])
        .unwrap()
        .encode(POP_OPCODE, &[2])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A16, 1211).unwrap();
    register.set_general(&Registers::B16, 2154).unwrap();
    register.set_sp(Address::new(255));
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::B16).unwrap(), 2154);
    assert_eq!(register.get_general(&Registers::A16).unwrap(), 1211);
}

#[test]
fn pop_u8() {
    let mut memory = Memory::new(256);
    let mut arg = [MOV_NUM2REG, 1].to_vec();
    arg.extend_from_slice(&111u8.to_le_bytes());
    let mut arg2 = [MOV_NUM2REG, 5].to_vec();
    arg2.extend_from_slice(&44u8.to_le_bytes());
    InstructionHelper::new(&mut memory)
        .encode(PUSH_OPCODE, &[1])
        .unwrap()
        .encode(PUSH_OPCODE, &[5])
        .unwrap()
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .encode(MOV_OPCODE, &arg2)
        .unwrap()
        .encode(POP_OPCODE, &[5])
        .unwrap()
        .encode(POP_OPCODE, &[1])
        .unwrap()
        .halt()
        .unwrap();
    let mut argument_memory = ArgumentMemory::new();
    let mut register = RegisterFile::new();
    register.set_general(&Registers::A8, 22).unwrap();
    register.set_general(&Registers::B8, 101).unwrap();
    register.set_sp(Address::new(255));
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
    drop(executor);
    assert_eq!(register.get_general(&Registers::B8).unwrap(), 101);
    assert_eq!(register.get_general(&Registers::A8).unwrap(), 22);
}
