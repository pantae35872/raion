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

//Cpu state releate instructions
pub const HALT_OPCODE: u16 = 65535;

// IO instructions
pub const OUTC_OPCODE: u16 = 128;

//Mov sub instructions
pub const MOV_REG2REG: u8 = 1;
pub const MOV_REG2MEM: u8 = 2;
pub const MOV_NUM2REG: u8 = 3;
pub const MOV_ADD2SP: u8 = 4;
pub const MOV_REG2SP: u8 = 5;
pub const MOV_DEREF_REG2REG: u8 = 6;
pub const MOV_SECTION_ADDR_2REG: u8 = 7;

pub const MAGIC_1: u8 = 69;
pub const MAGIC_2: u8 = 69;
pub const MAGIC_3: u8 = 0x69;
pub const MAGIC_4: u8 = 0x69;
