use std::{error::Error, fmt::Display};

use common::register::RegisterType;

use crate::memory::address::Address;

use self::flags::Flags;

mod flags;

/// Simple register file
///
/// Pattern:
/// "{register_name}{bits_amount}"
///
/// Registers:
/// 'a8' lower 8 bit of 'a' register
/// 'a16' lower 16 bit of 'a' register
/// 'a32' lower 32 bit of 'a' register
/// 'a64' full 64 bit of 'a' register
/// ...
/// 'ip' instruction pointer. bits amount depends on target arch
///
#[derive(Debug)]
pub struct RegisterFile {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    ip: Address,
    sp: Address,
    flags: Flags,
}

#[derive(Debug)]
pub enum RegisterFileError {
    SetError(RegisterType, u64),
    GeneralUnsupportSet(RegisterType),
}

impl Display for RegisterFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterFileError::SetError(register, value) => write!(
                f,
                "Trying to set a value to a register with oversize value: {}, register type: {}",
                value, register
            ),
            RegisterFileError::GeneralUnsupportSet(register) => write!(
                f,
                "Trying to access a non-general purpose register using general purpose register function. register: '{}'",
                register
            ),
        }
    }
}

impl Error for RegisterFileError {}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            ip: Address::new(0x0),
            sp: Address::new(0x0),
            flags: Flags::empty(),
        }
    }

    pub fn set_sp(&mut self, data: Address) {
        self.sp = data;
    }

    pub fn get_sp(&self) -> Address {
        return self.sp;
    }
    pub fn dec_sp(&mut self, amount: usize) -> Address {
        let value = self.sp - amount;
        self.sp = value;
        return value;
    }
    pub fn inc_sp(&mut self, amount: usize) {
        self.sp += amount;
    }

    pub fn set_ip(&mut self, data: Address) {
        self.ip = data;
    }

    pub fn get_ip(&self) -> Address {
        return self.ip;
    }

    /// Increment 'ip' by perfered value and return increased 'ip'
    pub fn inc_ip(&mut self, amount: usize) {
        self.ip += amount;
    }

    pub fn set_general(
        &mut self,
        register: &RegisterType,
        data: u64,
    ) -> Result<(), RegisterFileError> {
        match register {
            RegisterType::A8 | RegisterType::B8 | RegisterType::C8 | RegisterType::D8 => {
                match <u8>::try_from(data) {
                    Ok(_) => unsafe { self.set(register, data) },
                    Err(_) => return Err(RegisterFileError::SetError(register.clone(), data)),
                }
            }
            RegisterType::A16 | RegisterType::B16 | RegisterType::C16 | RegisterType::D16 => {
                match <u16>::try_from(data) {
                    Ok(_) => unsafe { self.set(register, data) },
                    Err(_) => return Err(RegisterFileError::SetError(register.clone(), data)),
                }
            }
            RegisterType::A32 | RegisterType::B32 | RegisterType::C32 | RegisterType::D32 => {
                match <u32>::try_from(data) {
                    Ok(_) => unsafe { self.set(register, data) },
                    Err(_) => return Err(RegisterFileError::SetError(register.clone(), data)),
                }
            }
            RegisterType::A64 | RegisterType::B64 | RegisterType::C64 | RegisterType::D64 => {
                unsafe { self.set(register, data) };
            }
            ur => return Err(RegisterFileError::GeneralUnsupportSet(ur.clone())),
        };
        return Ok(());
    }

    pub fn get_general(&self, register: &RegisterType) -> Result<u64, RegisterFileError> {
        match register {
            RegisterType::A8
            | RegisterType::B8
            | RegisterType::C8
            | RegisterType::D8
            | RegisterType::A16
            | RegisterType::B16
            | RegisterType::C16
            | RegisterType::D16
            | RegisterType::A32
            | RegisterType::B32
            | RegisterType::C32
            | RegisterType::D32
            | RegisterType::A64
            | RegisterType::B64
            | RegisterType::C64
            | RegisterType::D64 => return Ok(unsafe { self.get(register) }),
            ur => return Err(RegisterFileError::GeneralUnsupportSet(ur.clone())),
        };
    }

    pub unsafe fn get(&self, register: &RegisterType) -> u64 {
        match register {
            RegisterType::A8 => self.get_a8().into(),
            RegisterType::A16 => self.get_a16().into(),
            RegisterType::A32 => self.get_a32().into(),
            RegisterType::A64 => self.get_a64(),
            RegisterType::B8 => self.get_b8().into(),
            RegisterType::B16 => self.get_b16().into(),
            RegisterType::B32 => self.get_b32().into(),
            RegisterType::B64 => self.get_b64(),
            RegisterType::C8 => self.get_c8().into(),
            RegisterType::C16 => self.get_c16().into(),
            RegisterType::C32 => self.get_c32().into(),
            RegisterType::C64 => self.get_c64(),
            RegisterType::D8 => self.get_d8().into(),
            RegisterType::D16 => self.get_d16().into(),
            RegisterType::D32 => self.get_d32().into(),
            RegisterType::D64 => self.get_d64(),
            RegisterType::IP => self.get_ip().get_raw() as u64,
            RegisterType::SP => self.get_sp().get_raw() as u64,
            RegisterType::FLAGS => self.get_flags().bits().into(),
        }
    }

    pub unsafe fn set(&mut self, register: &RegisterType, data: u64) {
        match register {
            RegisterType::A8 => self.set_a8(data as u8),
            RegisterType::A16 => self.set_a16(data as u16),
            RegisterType::A32 => self.set_a32(data as u32),
            RegisterType::A64 => self.set_a64(data as u64),
            RegisterType::B8 => self.set_b8(data as u8),
            RegisterType::B16 => self.set_b16(data as u16),
            RegisterType::B32 => self.set_b32(data as u32),
            RegisterType::B64 => self.set_b64(data as u64),
            RegisterType::C8 => self.set_c8(data as u8),
            RegisterType::C16 => self.set_c16(data as u16),
            RegisterType::C32 => self.set_c32(data as u32),
            RegisterType::C64 => self.set_c64(data as u64),
            RegisterType::D8 => self.set_d8(data as u8),
            RegisterType::D16 => self.set_d16(data as u16),
            RegisterType::D32 => self.set_d32(data as u32),
            RegisterType::D64 => self.set_d64(data as u64),
            RegisterType::IP => self.set_ip(Address::new(data as usize)),
            RegisterType::SP => self.set_sp(Address::new(data as usize)),
            RegisterType::FLAGS => self.set_flags(Flags::from_bits_retain(data as u16)),
        }
    }

    pub fn set_flags(&mut self, data: Flags) {
        self.flags = data;
    }

    pub fn get_flags(&self) -> Flags {
        return self.flags;
    }

    pub fn get_negative(&self) -> bool {
        return self.flags.contains(Flags::NEGATIVE);
    }

    pub fn get_carry(&self) -> bool {
        return self.flags.contains(Flags::CARRY);
    }

    pub fn get_zero(&self) -> bool {
        return self.flags.contains(Flags::ZERO);
    }

    pub fn set_negative(&mut self, data: bool) {
        self.flags.set(Flags::NEGATIVE, data);
    }

    pub fn set_carry(&mut self, data: bool) {
        self.flags.set(Flags::CARRY, data);
    }

    pub fn set_zero(&mut self, data: bool) {
        self.flags.set(Flags::ZERO, data);
    }

    pub fn set_halt(&mut self, data: bool) {
        self.flags.set(Flags::HALT, data);
    }

    pub fn get_halt(&self) -> bool {
        return self.flags.contains(Flags::HALT);
    }

    fn set_a8(&mut self, data: u8) {
        self.a = (self.a & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_a16(&mut self, data: u16) {
        self.a = (self.a & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_a32(&mut self, data: u32) {
        self.a = (self.a & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_a64(&mut self, data: u64) {
        self.a = data;
    }

    fn get_a8(&self) -> u8 {
        return (self.a & 0xFF) as u8;
    }

    fn get_a16(&self) -> u16 {
        return (self.a & 0xFFFF) as u16;
    }

    fn get_a32(&self) -> u32 {
        return (self.a & 0xFFFFFFFF) as u32;
    }

    fn get_a64(&self) -> u64 {
        return self.a;
    }

    fn set_b8(&mut self, data: u8) {
        self.b = (self.b & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_b16(&mut self, data: u16) {
        self.b = (self.b & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_b32(&mut self, data: u32) {
        self.b = (self.b & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_b64(&mut self, data: u64) {
        self.b = data;
    }

    fn get_b8(&self) -> u8 {
        return (self.b & 0xFF) as u8;
    }

    fn get_b16(&self) -> u16 {
        return (self.b & 0xFFFF) as u16;
    }

    fn get_b32(&self) -> u32 {
        return (self.b & 0xFFFFFFFF) as u32;
    }

    fn get_b64(&self) -> u64 {
        return self.b;
    }

    fn set_c8(&mut self, data: u8) {
        self.c = (self.c & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_c16(&mut self, data: u16) {
        self.c = (self.c & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_c32(&mut self, data: u32) {
        self.c = (self.c & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_c64(&mut self, data: u64) {
        self.c = data;
    }

    fn get_c8(&self) -> u8 {
        return (self.c & 0xFF) as u8;
    }

    fn get_c16(&self) -> u16 {
        return (self.c & 0xFFFF) as u16;
    }

    fn get_c32(&self) -> u32 {
        return (self.c & 0xFFFFFFFF) as u32;
    }

    fn get_c64(&self) -> u64 {
        return self.c;
    }

    fn set_d8(&mut self, data: u8) {
        self.d = (self.d & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    fn set_d16(&mut self, data: u16) {
        self.d = (self.d & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    fn set_d32(&mut self, data: u32) {
        self.d = (self.d & 0xFFFFFFFF00000000) | data as u64;
    }

    fn set_d64(&mut self, data: u64) {
        self.d = data;
    }

    fn get_d8(&self) -> u8 {
        return (self.d & 0xFF) as u8;
    }

    fn get_d16(&self) -> u16 {
        return (self.d & 0xFFFF) as u16;
    }

    fn get_d32(&self) -> u32 {
        return (self.d & 0xFFFFFFFF) as u32;
    }

    fn get_d64(&self) -> u64 {
        return self.d;
    }
}
