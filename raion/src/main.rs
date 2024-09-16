use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

use common::sin::Sin;
use raion::{compiler::asm_compiler::ASMCompiler, lexer::Lexer};

fn main() -> ExitCode {
    let mut file = File::open("in.asm").expect("file not found");
    let mut abc = String::new();
    file.read_to_string(&mut abc).unwrap();
    let lexer = Lexer::new(&abc);
    let tokens = lexer.tokenize_asm().expect("Cannot parse");
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
