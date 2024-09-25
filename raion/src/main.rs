use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

use common::{
    commands::{Command, CommandExecutor},
    sin::Sin,
};
use raion::{
    compiler::{
        asm_compiler::ASMCompiler,
        rin_compiler::{Path as RinPath, RinCompiler},
    },
    lexer::{asm_lexer::ASMLexer, rin_lexer::RinLexer},
    manager::SingleUseCompiler,
};

fn command_run(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    todo!()
}

fn command_build(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    todo!()
}

fn command_compile_emit_asm(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    let file_name = args.next().ok_or("no rin file is provided".to_string())?;
    let source_path = Path::new(&file_name);
    let package_name = source_path
        .file_stem()
        .ok_or("File name is not provided".to_string())?
        .to_str()
        .ok_or(format!("file name is not valid utf8"))?;
    let output = source_path.with_extension("sin");
    let output_asm = source_path.with_extension("asm");
    let module_path = RinPath::new(package_name);
    let mut compiler = SingleUseCompiler::new(
        source_path,
        &output,
        Some(&output_asm),
        &module_path,
        package_name,
    );
    compiler
        .prepare_buffer()
        .map_err(|e| format!("couldn't read {}: {e}", source_path.display()))?;
    compiler.generate()?;
    return Ok(());
}

fn command_compile(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    let file_name = args.next().ok_or("no rin file is provided".to_string())?;
    let source_path = Path::new(&file_name);
    let package_name = source_path
        .file_stem()
        .ok_or("File name is not provided".to_string())?
        .to_str()
        .ok_or(format!("file name is not valid utf8"))?;
    let output = source_path.with_extension("sin");
    let module_path = RinPath::new(package_name);
    let mut compiler =
        SingleUseCompiler::new(source_path, &output, None, &module_path, package_name);
    compiler
        .prepare_buffer()
        .map_err(|e| format!("couldn't read {}: {e}", source_path.display()))?;
    compiler.generate()?;
    return Ok(());
}

fn main() -> ExitCode {
    return CommandExecutor::new()
        .new_command(Command::new(
            "run",
            "build and run the project in the current directory",
            "",
            command_run,
        ))
        .new_command(Command::new(
            "build",
            "build the project in the current directory",
            "",
            command_build,
        ))
        .new_command(Command::new(
            "compile",
            "compile the provided rin file",
            "<sin file>",
            command_compile,
        ))
        .new_command(Command::new(
            "compile-emit-asm",
            "compile the provided sin file and emit asm",
            "<sin file>",
            command_compile_emit_asm,
        ))
        .run();
}
