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
};

fn command_run(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    todo!()
}

fn command_build(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    todo!()
}

fn command_compile(_command_name: &str, args: &mut env::Args) -> Result<(), String> {
    let file_name = args.next().ok_or("no rin file is provided".to_string())?;
    let file_path = Path::new(&file_name);
    let mut file = File::open(&file_path)
        .map_err(|e| format!("couldn't read {}: {e}", file_path.display()))?;
    let mut data = String::new();
    file.read_to_string(&mut data)
        .map_err(|e| format!("couldn't read {}. {e}", file_path.display()))?;
    let lexer = RinLexer::new(&data, file_path);
    let tokens = match lexer.tokenize() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return Err(format!(
                "could not compile `{}` due to previous lexer error",
                file_path.display()
            ));
        }
    };
    let mut compiler = RinCompiler::new(
        tokens,
        RinPath::new(
            file_path
                .file_stem()
                .ok_or("File name is not provided".to_string())?
                .to_str()
                .ok_or(format!("file name is not valid utf8"))?,
        ),
    );
    match compiler.parse() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{err}");
            return Err(format!(
                "could not compile `{}` due to previous parsing error",
                file_path.display()
            ));
        }
    };
    let generated_asm = match compiler.generate() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return Err(format!(
                "could not compile `{}` due to previous generation error",
                file_path.display()
            ));
        }
    };

    if Path::new("out.asm").exists() {
        fs::remove_file("out.asm")
            .map_err(|e| format!("couldn't remove existing out.asm file: {e}"))?;
    }

    let mut file =
        File::create("out.asm").map_err(|e| format!("cannot create out.asm file: {e}"))?;
    write!(file, "{generated_asm}").map_err(|e| format!("cannot write to out.asm: {e}"))?;

    let lexer = ASMLexer::new(&generated_asm, file_path);
    let tokens = match lexer.tokenize() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return Err(format!(
                "could not compile `{}` due to previous byte code generation error",
                file_path.display()
            ));
        }
    };
    let compiler = ASMCompiler::new(tokens);
    match compiler.compile() {
        Ok((sections, data)) => {
            let sin = Sin::new(sections, &data).to_bytes();

            if Path::new("out.sin").exists() {
                fs::remove_file("out.sin")
                    .map_err(|e| format!("couldn't remove existing out.asm file: {e}"))?;
            }
            File::create("out.sin")
                .map_err(|e| format!("cannot create out.sin file: {e}"))?
                .write_all(&sin)
                .map_err(|e| format!("cannot write to out.sin: {e}"))?;
        }
        Err(err) => {
            eprintln!("{err}");
            return Err(format!(
                "could not compile `{}` due to previous byte code generation error",
                file_path.display()
            ));
        }
    };

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
        .run();
}
