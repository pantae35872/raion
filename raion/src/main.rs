use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

use common::sin::Sin;
use raion::{
    compiler::{
        asm_compiler::ASMCompiler,
        rin_compiler::{Path as RinPath, RinCompiler},
    },
    lexer::{asm_lexer::ASMLexer, rin_lexer::RinLexer},
};

fn main() -> ExitCode {
    let mut file = File::open("in.rin").expect("file not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let lexer = RinLexer::new(&data, Path::new("in.rin"));
    let tokens = match lexer.tokenize() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };
    let mut compiler = RinCompiler::new(tokens, RinPath::new("main"));
    match compiler.parse() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };
    let generated_asm = match compiler.generate() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    if Path::new("out.asm").exists() {
        fs::remove_file("out.asm").unwrap();
    }
    let mut file = File::create("out.asm").expect("UNNN");
    write!(file, "{generated_asm}").unwrap();
    let lexer = ASMLexer::new(&generated_asm, Path::new("in.rin"));
    let tokens = lexer.tokenize().expect("Cannot parse");
    let compiler = ASMCompiler::new(tokens);
    match compiler.compile() {
        Ok((sections, data)) => {
            let sin = Sin::new(sections, &data).to_bytes();

            if Path::new("out.sin").exists() {
                fs::remove_file("out.sin").unwrap();
            }
            if let Ok(mut file) = File::create("out.sin") {
                file.write_all(&sin).unwrap();
            } else {
                eprintln!("Failed to create out.sin file");
                return ExitCode::FAILURE;
            }
        }
        Err(er) => {
            eprintln!("{}", er);
            return ExitCode::FAILURE;
        }
    };
    return ExitCode::SUCCESS;
}
