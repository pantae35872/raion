mod instruction_helper;

use common::constants::{MOV_OPCODE, MOV_REG2MEM, MOV_REG2REG};
use craion::{
    executor::{registers::Registers, Executor},
    memory::address::Address,
};
use instruction_helper::InstructionHelper;

#[test]
fn reg2reg() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE, &[MOV_REG2REG, 4, 8])
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A64, 32)
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::B64, 64)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&Registers::A64).unwrap(),
        executor.registers().get_general(&Registers::B64).unwrap(),
    );
}

#[test]
fn reg2mem_u8() {
    let mut executor = Executor::new(0xFFFF);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(63).get_raw().to_le_bytes());
    arg.push(1);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A8, 32)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&Registers::A8)
            .unwrap()
            .to_le_bytes()[0],
        executor.memory().mem_get(Address::new(63)).unwrap()
    );
}

#[test]
fn reg2mem_u16() {
    let mut executor = Executor::new(0xFFFF);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(61).get_raw().to_le_bytes());
    arg.push(2);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A16, 32)
        .unwrap();
    executor.execute();
    assert_eq!(
        &executor
            .registers()
            .get_general(&Registers::A16)
            .unwrap()
            .to_le_bytes()[0..2],
        executor.memory().mem_gets(Address::new(61), 2).unwrap()
    );
}

#[test]
fn reg2mem_u32() {
    let mut executor = Executor::new(0xFFFF);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(59).get_raw().to_le_bytes());
    arg.push(3);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A32, 32)
        .unwrap();
    executor.execute();
    assert_eq!(
        &executor
            .registers()
            .get_general(&Registers::A32)
            .unwrap()
            .to_le_bytes()[0..4],
        executor.memory().mem_gets(Address::new(59), 4).unwrap()
    );
}

#[test]
fn reg2mem_u64() {
    let mut executor = Executor::new(0xFFFF);
    let mut arg = [MOV_REG2MEM].to_vec();
    arg.extend_from_slice(&Address::new(55).get_raw().to_le_bytes());
    arg.push(4);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE, &arg)
        .unwrap()
        .halt()
        .unwrap();
    executor
        .registers()
        .set_general(&Registers::A64, 32)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&Registers::A64)
            .unwrap()
            .to_le_bytes(),
        executor.memory().mem_gets(Address::new(55), 8).unwrap()
    );
}
