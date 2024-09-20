//Memory releate instructions
pub const MOV_OPCODE: u16 = 16;
pub const PUSH_OPCODE: u16 = 17;
pub const POP_OPCODE: u16 = 18;

//Arithmetic instructions
pub const INC_OPCODE: u16 = 30;
pub const CMP_OPCODE: u16 = 31;
pub const ADD_OPCODE: u16 = 32;
pub const SUB_OPCODE: u16 = 33;

//Branching instructions
pub const JMP_OPCODE: u16 = 64;
pub const JMZ_OPCODE: u16 = 65;
pub const JMN_OPCODE: u16 = 66;
pub const JACN_OPCODE: u16 = 67;
pub const JACZ_OPCODE: u16 = 68;
pub const JACC_OPCODE: u16 = 69;
pub const JACE_OPCODE: u16 = 70;
pub const JME_OPCODE: u16 = 71;
pub const JMC_OPCODE: u16 = 72;
pub const CALL_OPCODE: u16 = 73;
pub const RET_OPCODE: u16 = 74;

//Cpu state releate instructions
pub const HALT_OPCODE: u16 = 65535;

// IO instructions
pub const OUTC_OPCODE: u16 = 128;

//Mov sub instructions
pub const MOV_REG2REG: u8 = 1;
pub const MOV_REG2DEREF_REG: u8 = 2;
pub const MOV_NUM2REG: u8 = 3;
pub const MOV_ADD2SP: u8 = 4;
pub const MOV_REG2SP: u8 = 5;
pub const MOV_DEREF_REG2REG: u8 = 6;
pub const MOV_SECTION_ADDR_2REG: u8 = 7;
pub const MOV_NUM2DEREF_REG: u8 = 8;
pub const MOV_NUM2DEREF_REG_WITH_ADD_OFFSET: u8 = 9;
pub const MOV_NUM2DEREF_REG_WITH_SUB_OFFSET: u8 = 10;
pub const MOV_NUM2DEREF_SP_WITH_ADD_OFFSET: u8 = 11;
pub const MOV_NUM2DEREF_SP_WITH_SUB_OFFSET: u8 = 12;
pub const MOV_DEREF_SP_WITH_SUB_OFFSET2DEREF_SP_WITH_SUB_OFFSET_WITH_SIZE: u8 = 13;
pub const MOV_DEREF_SP_WITH_ADD_OFFSET2DEREF_SP_WITH_SUB_OFFSET_WITH_SIZE: u8 = 14;
pub const MOV_DEREF_SP_WITH_SUB_OFFSET2DEREF_SP_WITH_ADD_OFFSET_WITH_SIZE: u8 = 15;
pub const MOV_DEREF_SP_WITH_ADD_OFFSET2DEREF_SP_WITH_ADD_OFFSET_WITH_SIZE: u8 = 16;
pub const MOV_DEREF_REG_WITH_SUB_OFFSET2DEREF_REG_WITH_SUB_OFFSET_WITH_SIZE: u8 = 17;
pub const MOV_DEREF_REG_WITH_ADD_OFFSET2DEREF_REG_WITH_SUB_OFFSET_WITH_SIZE: u8 = 18;
pub const MOV_DEREF_REG_WITH_SUB_OFFSET2DEREF_REG_WITH_ADD_OFFSET_WITH_SIZE: u8 = 19;
pub const MOV_DEREF_REG_WITH_ADD_OFFSET2DEREF_REG_WITH_ADD_OFFSET_WITH_SIZE: u8 = 20;
pub const MOV_REG2DEREF_SP_WITH_SUB_OFFSET: u8 = 21;
pub const MOV_REG2DEREF_SP_WITH_ADD_OFFSET: u8 = 22;
pub const MOV_REG2DEREF_REG_WITH_ADD_OFFSET: u8 = 23;
pub const MOV_REG2DEREF_REG_WITH_SUB_OFFSET: u8 = 24;
pub const MOV_SECTION_ADDR2DEREF_SP_WITH_SUB_OFFSET: u8 = 25;
pub const MOV_SECTION_ADDR2DEREF_SP_WITH_ADD_OFFSET: u8 = 26;
pub const MOV_SECTION_ADDR2DEREF_REG_WITH_ADD_OFFSET: u8 = 27;
pub const MOV_SECTION_ADDR2DEREF_REG_WITH_SUB_OFFSET: u8 = 28;
pub const MOV_DEREF_SP_WITH_SUB_OFFSET2REG: u8 = 29;
pub const MOV_DEREF_SP_WITH_ADD_OFFSET2REG: u8 = 30;

// Sub sub instructions
pub const SUB_REG_W_REG: u8 = 1;
pub const SUB_REG_W_NUM: u8 = 2;
pub const SUB_SP_W_NUM: u8 = 3;

// Add sub instructions
pub const ADD_REG_W_REG: u8 = 1;
pub const ADD_REG_W_NUM: u8 = 2;
pub const ADD_SP_W_NUM: u8 = 3;

pub const MAGIC_1: u8 = 69;
pub const MAGIC_2: u8 = 69;
pub const MAGIC_3: u8 = 0x69;
pub const MAGIC_4: u8 = 0x69;
