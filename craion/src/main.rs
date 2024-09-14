#![deny(warnings)]
#![feature(test)]

use std::env;
use std::fs::File;
use std::io::Read;

use common::sin::Sin;
use craion::executor::{registers::RegisterFile, Executor};
use craion::memory::{address::Address, argument_memory::ArgumentMemory};

use craion::memory::Memory;

extern crate test;

fn main() {
    let mut memory = Memory::new(0xFFFF);
    let args: Vec<String> = env::args().collect();
    let mut sin = File::open(args.get(1).expect("Provied sin file to be execute"))
        .expect("The sin file does not exists");
    let mut buf = Vec::new();
    sin.read_to_end(&mut buf).unwrap();
    let sin = Sin::from_bytes(&buf).expect("");
    memory.mem_sets(Address::new(0), sin.text()).expect("");
    let mut register = RegisterFile::new();
    let mut argument_memory = ArgumentMemory::new();
    let mut executor = Executor::new(&mut memory, &mut register, &mut argument_memory);
    executor.execute();
}
