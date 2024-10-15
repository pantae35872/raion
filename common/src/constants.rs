use xxhash_rust::const_xxh3;

//Memory releate instructions
pub const MOV_OPCODE: u16 = 16;
pub const PUSH_OPCODE: u16 = 17;
pub const POP_OPCODE: u16 = 18;
pub const ENTER_OPCODE: u16 = 19;
pub const LEAVE_OPCODE: u16 = 20;
pub const ARG_OPCODE: u16 = 21;
pub const LARG_OPCODE: u16 = 22;
pub const SAVR_OPCODE: u16 = 23;
pub const RESTR_OPCODE: u16 = 24;

//Arithmetic instructions
pub const INC_OPCODE: u16 = 30;
pub const CMP_OPCODE: u16 = 31;
pub const ADD_OPCODE: u16 = 32;
pub const SUB_OPCODE: u16 = 33;
pub const MUL_OPCODE: u16 = 34;
pub const DIV_OPCODE: u16 = 35;

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
pub const EXIT_OPCODE: u16 = 65534;
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
pub const MOV_NUM2DEREF_REG_WITH_OFFSET: u8 = 9;
pub const MOV_REG2DEREF_REG_WITH_OFFSET: u8 = 10;
pub const MOV_DEREF_REG_WITH_OFFSET2REG: u8 = 11;
pub const MOV_SECTION_ADDR2DEREF_REG_WITH_OFFSET: u8 = 12;

// Sub sub instructions
pub const SUB_REG_W_REG: u8 = 1;
pub const SUB_REG_W_NUM: u8 = 2;
pub const SUB_SP_W_NUM: u8 = 3;

// Add sub instructions
pub const ADD_REG_W_REG: u8 = 1;
pub const ADD_REG_W_NUM: u8 = 2;
pub const ADD_SP_W_NUM: u8 = 3;

// Arg sub instructions
pub const ARG_NUM: u8 = 1;
pub const ARG_REG: u8 = 2;

// Sin format
pub const MAGIC_1: u8 = 69;
pub const MAGIC_2: u8 = 69;
pub const MAGIC_3: u8 = 0x69;
pub const MAGIC_4: u8 = 0x69;
pub const MAJOR: u8 = 1;
pub const MINOR: u8 = 0;
pub const PATCH: u8 = 0;

// ALl the primitive types hash
pub const U8_HASH: u64 = const_xxh3::xxh3_64(b"u8");
pub const U16_HASH: u64 = const_xxh3::xxh3_64(b"u16");
pub const U32_HASH: u64 = const_xxh3::xxh3_64(b"u32");
pub const U64_HASH: u64 = const_xxh3::xxh3_64(b"u64");
pub const I8_HASH: u64 = const_xxh3::xxh3_64(b"i8");
pub const I16_HASH: u64 = const_xxh3::xxh3_64(b"i16");
pub const I32_HASH: u64 = const_xxh3::xxh3_64(b"i32");
pub const I64_HASH: u64 = const_xxh3::xxh3_64(b"i64");
pub const BOOL_HASH: u64 = const_xxh3::xxh3_64(b"bool");
pub const VOID_HASH: u64 = const_xxh3::xxh3_64(b"void");
