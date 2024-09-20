use common::{
    constants::{MOV_NUM2REG, MOV_OPCODE, MOV_REG2DEREF_REG, MOV_REG2REG},
    register::RegisterType,
};
use craion::{executor::Executor, instruction_helper::InstructionHelper, memory::address::Address};

#[test]
fn reg2reg() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_REG2REG)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 32)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 64)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        executor
            .registers()
            .get_general(&RegisterType::B64)
            .unwrap(),
    );
}

#[test]
fn num2reg_u8() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A8)
        .encode_u8(211)
        .end()
        .halt();
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&RegisterType::A8).unwrap(),
        211
    );
}

#[test]
fn num2reg_u16() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A16)
        .encode_u16(2211)
        .end()
        .halt();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A16)
            .unwrap(),
        2211
    );
}

#[test]
fn num2reg_u32() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A32)
        .encode_u32(2211520)
        .end()
        .halt();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A32)
            .unwrap(),
        2211520
    );
}

#[test]
fn num2reg_u64() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A64)
        .encode_u64(22115221320)
        .end()
        .halt();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        22115221320
    );
}

#[test]
fn reg2mem_u8() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_REG2DEREF_REG)
        .encode_register(RegisterType::A8)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A8, 120)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 0xFF)
        .unwrap();
    executor.execute();
    assert_eq!(
        &executor
            .registers()
            .get_general(&RegisterType::A8)
            .unwrap()
            .to_le_bytes()[0..1],
        executor.memory().mem_gets(Address::new(0xFF), 1).unwrap()
    );
}

#[test]
fn reg2mem_u16() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_REG2DEREF_REG)
        .encode_register(RegisterType::A16)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A16, 65512)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 0xFF)
        .unwrap();
    executor.execute();
    assert_eq!(
        &executor
            .registers()
            .get_general(&RegisterType::A16)
            .unwrap()
            .to_le_bytes()[0..2],
        executor.memory().mem_gets(Address::new(0xFF), 2).unwrap()
    );
}

#[test]
fn reg2mem_u32() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_REG2DEREF_REG)
        .encode_register(RegisterType::A32)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A32, 45555)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 0xFF)
        .unwrap();
    executor.execute();
    assert_eq!(
        &executor
            .registers()
            .get_general(&RegisterType::A32)
            .unwrap()
            .to_le_bytes()[0..4],
        executor.memory().mem_gets(Address::new(0xFF), 4).unwrap()
    );
}

#[test]
fn reg2deref_reg_u64() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_REG2DEREF_REG)
        .encode_register(RegisterType::A64)
        .encode_register(RegisterType::B64)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A64, 15)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B64, 0xFF)
        .unwrap();
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap()
            .to_le_bytes(),
        executor.memory().mem_gets(Address::new(0xFF), 8).unwrap()
    );
}
