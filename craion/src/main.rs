#![deny(warnings)]
#![feature(test)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use common::sin::sections::SectionType;
use common::sin::Sin;
use craion::executor::{registers::RegisterFile, Executor};
use craion::memory::{address::Address, argument_memory::ArgumentMemory};

use craion::memory::Memory;
use craion::ret_stack::RetStack;
use craion::section_manager::SectionManager;

extern crate test;

fn main() -> ExitCode {
    let mut memory = Memory::new(0xFFFF);
    let args: Vec<String> = env::args().collect();
    let mut sin = File::open(args.get(1).expect("Provied sin file to be execute"))
        .expect("The sin file does not exists");
    let mut buf = Vec::new();
    sin.read_to_end(&mut buf).unwrap();
    let sin = Sin::from_bytes(&buf).expect("");
    let mut section_manager = SectionManager::new();
    for section in sin.sections() {
        section_manager.load_section(section, sin.data(), &mut memory);
    }
    let mut register = RegisterFile::new();
    if let Some(entry) = section_manager.get_section("start") {
        if entry.section_type() != SectionType::Function {
            eprintln!("Entry point is not a function");
            return ExitCode::FAILURE;
        }
        register.set_ip(entry.mem_start());
    } else {
        eprintln!("Entry point not found");
        return ExitCode::FAILURE;
    }
    register.set_sp(Address::new(0xFFFE));
    let mut argument_memory = ArgumentMemory::new();
    let mut ret_stack = RetStack::new();
    let mut executor = Executor::new(
        &mut memory,
        &mut register,
        &mut argument_memory,
        &mut ret_stack,
        &mut section_manager,
    );
    executor.execute();
    return ExitCode::SUCCESS;
}
