#![deny(warnings)]
#![feature(test)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use common::sin::sections::SectionType;
use common::sin::Sin;
use craion::executor::Executor;
use craion::memory::address::Address;

extern crate test;

fn main() -> ExitCode {
    let mut executor = Executor::new(0xFFFF);
    let args: Vec<String> = env::args().collect();
    let mut sin = File::open(args.get(1).expect("Provied sin file to be execute"))
        .expect("The sin file does not exists");
    let mut buf = Vec::new();
    sin.read_to_end(&mut buf).unwrap();
    let sin = Sin::from_bytes(&buf).expect("");
    for section in sin.sections() {
        executor.load_section(section, sin.data());
    }
    let entry = if let Some(entry) = executor.section_manager().get_section("start") {
        if entry.section_type() != SectionType::Procedure {
            eprintln!("Entry point is not a procedure");
            return ExitCode::FAILURE;
        }
        entry.mem_start()
    } else {
        eprintln!("Entry point not found");
        return ExitCode::FAILURE;
    };
    executor.registers().set_ip(entry);
    executor.registers().set_sp(Address::new(0xFFFE));
    executor.execute();
    return ExitCode::SUCCESS;
}
