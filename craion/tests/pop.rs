use common::{
    constants::{MOV_NUM2REG, MOV_OPCODE, POP_OPCODE, PUSH_OPCODE},
    register::RegisterType,
};
use craion::{executor::Executor, instruction_helper::InstructionHelper, memory::address::Address};

#[test]
fn pop_u64() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A64)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B64)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A64)
        .encode_u64(123980)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::B64)
        .encode_u64(943099)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::B64)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::A64)
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
        687545
    );
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A64)
            .unwrap(),
        33
    );
}

#[test]
fn pop_u32() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A32)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B32)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A32)
        .encode_u32(642)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::B32)
        .encode_u32(4454)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::B32)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::A32)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A32, 1211)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B32, 2154)
        .unwrap();
    executor.registers().set_sp(Address::new(0xFFFE));
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::B32)
            .unwrap(),
        2154
    );
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A32)
            .unwrap(),
        1211
    );
}
#[test]
fn pop_u16() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A16)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B16)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A16)
        .encode_u32(642)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::B16)
        .encode_u32(4454)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::B16)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::A16)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A16, 1211)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B16, 2154)
        .unwrap();
    executor.registers().set_sp(Address::new(0xFFFE));
    executor.execute();
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::B16)
            .unwrap(),
        2154
    );
    assert_eq!(
        executor
            .registers()
            .get_general(&RegisterType::A16)
            .unwrap(),
        1211
    );
}

#[test]
fn pop_u8() {
    let mut executor = Executor::new(0xFFFF);
    InstructionHelper::new(&mut executor.memory())
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::A8)
        .end()
        .encode(PUSH_OPCODE)
        .encode_register(RegisterType::B8)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::A8)
        .encode_u32(44)
        .end()
        .encode(MOV_OPCODE)
        .encode_sub_opcode(MOV_NUM2REG)
        .encode_register(RegisterType::B8)
        .encode_u32(111)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::B8)
        .end()
        .encode(POP_OPCODE)
        .encode_register(RegisterType::A8)
        .end()
        .halt();
    executor
        .registers()
        .set_general(&RegisterType::A8, 22)
        .unwrap();
    executor
        .registers()
        .set_general(&RegisterType::B8, 101)
        .unwrap();
    executor.registers().set_sp(Address::new(0xFFFE));
    executor.execute();
    assert_eq!(
        executor.registers().get_general(&RegisterType::B8).unwrap(),
        101
    );
    assert_eq!(
        executor.registers().get_general(&RegisterType::A8).unwrap(),
        22
    );
}
