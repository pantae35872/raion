use std::ops::Add;

use crate::memory::address::Address;

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
pub struct Register {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    ip: Address,
}

impl Register {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            ip: Address::new(0x0),
        }
    }

    pub fn set_ip(&mut self, data: Address) {
        self.ip = data;
    }

    pub fn get_ip(&self) -> Address {
        return self.ip.clone();
    }

    /// Increment 'ip' by perfered value and return increased 'ip'
    pub fn inc_ip(&mut self, amount: usize) -> Address {
        let inc_ip = self.ip.clone() + amount;
        self.ip = inc_ip.clone();
        return inc_ip;
    }

    pub fn set_a8(&mut self, data: u8) {
        self.a = (self.a & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    pub fn set_a16(&mut self, data: u16) {
        self.a = (self.a & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    pub fn set_a32(&mut self, data: u32) {
        self.a = (self.a & 0xFFFFFFFF00000000) | data as u64;
    }

    pub fn set_a64(&mut self, data: u64) {
        self.a = data;
    }

    pub fn get_a8(&self) -> u8 {
        return (self.a & 0xFF) as u8;
    }

    pub fn get_a16(&self) -> u16 {
        return (self.a & 0xFFFF) as u16;
    }

    pub fn get_a32(&self) -> u32 {
        return (self.a & 0xFFFFFFFF) as u32;
    }

    pub fn get_a64(&self) -> u64 {
        return self.a;
    }

    pub fn set_b8(&mut self, data: u8) {
        self.b = (self.b & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    pub fn set_b16(&mut self, data: u16) {
        self.b = (self.b & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    pub fn set_b32(&mut self, data: u32) {
        self.b = (self.b & 0xFFFFFFFF00000000) | data as u64;
    }

    pub fn set_b64(&mut self, data: u64) {
        self.b = data;
    }

    pub fn get_b8(&self) -> u8 {
        return (self.b & 0xFF) as u8;
    }

    pub fn get_b16(&self) -> u16 {
        return (self.b & 0xFFFF) as u16;
    }

    pub fn get_b32(&self) -> u32 {
        return (self.b & 0xFFFFFFFF) as u32;
    }

    pub fn get_b64(&self) -> u64 {
        return self.b;
    }

    pub fn set_c8(&mut self, data: u8) {
        self.c = (self.c & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    pub fn set_c16(&mut self, data: u16) {
        self.c = (self.c & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    pub fn set_c32(&mut self, data: u32) {
        self.c = (self.c & 0xFFFFFFFF00000000) | data as u64;
    }

    pub fn set_c64(&mut self, data: u64) {
        self.c = data;
    }

    pub fn get_c8(&self) -> u8 {
        return (self.c & 0xFF) as u8;
    }

    pub fn get_c16(&self) -> u16 {
        return (self.c & 0xFFFF) as u16;
    }

    pub fn get_c32(&self) -> u32 {
        return (self.c & 0xFFFFFFFF) as u32;
    }

    pub fn get_c64(&self) -> u64 {
        return self.c;
    }

    pub fn set_d8(&mut self, data: u8) {
        self.d = (self.d & 0xFFFFFFFFFFFFFF00) | data as u64;
    }

    pub fn set_d16(&mut self, data: u16) {
        self.d = (self.d & 0xFFFFFFFFFFFF0000) | data as u64;
    }

    pub fn set_d32(&mut self, data: u32) {
        self.d = (self.d & 0xFFFFFFFF00000000) | data as u64;
    }

    pub fn set_d64(&mut self, data: u64) {
        self.d = data;
    }

    pub fn get_d8(&self) -> u8 {
        return (self.d & 0xFF) as u8;
    }

    pub fn get_d16(&self) -> u16 {
        return (self.d & 0xFFFF) as u16;
    }

    pub fn get_d32(&self) -> u32 {
        return (self.d & 0xFFFFFFFF) as u32;
    }

    pub fn get_d64(&self) -> u64 {
        return self.d;
    }
}
