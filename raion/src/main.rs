use std::{env, fs::File, io::Read, path::Path, process::ExitCode};

use common::commands::{Command, CommandExecutor};
use raion::{
    compiler::rin_compiler::Path as RinPath,
    manager::{CompilerManager, SingleUseCompiler},
};
use toml::Value;

fn command_run(_command_name: &str, _args: &mut env::Args) -> Result<(), String> {
    todo!()
}

fn command_build(_command_name: &str, _args: &mut env::Args) -> Result<(), String> {
    let working_dir = std::env::current_dir().map_err(|e| format!("can't get current dir: {e}"))?;
    let mut config = File::open(working_dir.join("config.toml"))
        .map_err(|e| format!("failed to open 'config.toml' in the current directory: {e}"))?;
    let mut config_buf = String::new();
    config
        .read_to_string(&mut config_buf)
        .map_err(|err| format!("failed to read 'config.file': {err}"))?;
    let config =
        toml::from_str::<Value>(&config_buf).map_err(|e| format!("invalid config file: {e}"))?;
    let package = config
        .get("package")
        .ok_or(format!("no package table in config file"))?;
    let name = package
        .get("name")
        .ok_or(format!("no name in the package tabel"))?
        .as_str()
        .ok_or(format!("the provided name is not a string"))?;
    let mut manager = CompilerManager::new(&working_dir, name);
    manager.parse_files()?;
    manager.generate()?;
    return Ok(());
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
            "<rin file>",
            command_compile,
        ))
        .new_command(Command::new(
            "compile-emit-asm",
            "compile the provided sin file and emit asm",
            "<rin file>",
            command_compile_emit_asm,
        ))
        .run();
}
