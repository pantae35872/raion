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
    compiler::{asm_compiler::ASMCompiler, rin_compiler::Path as RinPath},
    lexer::asm_lexer::ASMLexer,
    manager::{CompilerManager, SingleUseCompiler},
    WithLocation,
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

fn command_compile_rasm(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    let file_name = args.next().ok_or("no rasm file is provided".to_string())?;
    let source_path = Path::new(&file_name);
    let package_name = source_path
        .file_stem()
        .ok_or("File name is not provided".to_string())?
        .to_str()
        .ok_or(format!("file name is not valid utf8"))?;
    let output = source_path.with_extension("sin");
    let output_asm = source_path.with_extension("asm");
    let module_path = RinPath::new(package_name);
    let mut file = File::open(source_path)
        .map_err(|e| format!("couldn't read {}: {e}", source_path.display()))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .map_err(|e| format!("couldn't read {}: {e}", source_path.display()))?;
    let lexer = ASMLexer::new(&buffer, source_path.into());
    let tokens = match lexer.tokenize() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return Err(format!(
                "could not compile `{}` due to previous byte code generation error",
                package_name
            ));
        }
    };
    let compiler = ASMCompiler::new(tokens);
    match compiler.compile() {
        Ok((sections, data)) => {
            let sin = Sin::new(sections, &data).to_bytes();

            if Path::new(&output).exists() {
                fs::remove_file(&output)
                    .map_err(|e| format!("couldn't remove existing sin file: {e}"))?;
            }
            File::create(output)
                .map_err(|e| format!("cannot create sin file: {e}"))?
                .write_all(&sin)
                .map_err(|e| format!("cannot write to sin: {e}"))?;
        }
        Err(err) => {
            eprintln!("{err}");
            return Err(format!(
                "could not compile `{}` due to previous byte code generation error",
                package_name
            ));
        }
    };

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
            "compile-emit-rasm",
            "compile the provided sin file and emit rasm",
            "<rin file>",
            command_compile_emit_asm,
        ))
        .new_command(Command::new(
            "compile-rasm",
            "compile the provided rasm",
            "<rasm file>",
            command_compile_rasm,
        ))
        .run();
}
