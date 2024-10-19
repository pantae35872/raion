#![feature(test)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use common::commands::{Command, CommandExecutor};
use common::sin::Sin;
use craion::executor::objects::{Object, Primitive};
use craion::executor::Executor;
use craion::section_manager::LoadedType;
use xxhash_rust::const_xxh3;

extern crate test;

fn command_run(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    let mut executor = Executor::new(0xFF);
    let file = args.next().ok_or("no sin file is provided".to_string())?;
    let mut sin = File::open(&file).map_err(|e| format!("couldn't read {file}: {e}"))?;
    let mut buf = Vec::new();
    sin.read_to_end(&mut buf)
        .map_err(|e| format!("failed to read {file}: {e}"))?;
    let sin =
        Sin::from_bytes(&buf).map_err(|e| format!("couldn't parse the provided sin file: {e}"))?;
    for section in sin.sections() {
        executor.load_section(section, sin.data());
    }
    executor.init();
    //let entry = if let Some(entry) = executor.section_manager()("start") {
    //    if entry.section_type() != SectionType::Procedure {
    //        return Err("entry point is not a procedure".to_string());
    //    }
    //    entry.mem_start()
    //} else {
    //    return Err("entry point not found".to_string());
    //};
    //executor.registers().set_ip(entry);
    //executor.registers().set_sp(Address::new(0xFFFE));
    executor.execute();
    return Ok(());
}

fn main() -> ExitCode {
    return CommandExecutor::new()
        .new_command(Command::new(
            "run",
            "run the provided sin file",
            "<sin_file>",
            command_run,
        ))
        .run();
}
