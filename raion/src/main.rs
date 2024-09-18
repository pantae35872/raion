use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

use common::sin::Sin;
use raion::{
    compiler::asm_compiler::ASMCompiler,
    lexer::{asm_lexer::ASMLexer, rin_lexer::RinLexer},
};

fn main() -> ExitCode {
    let mut file = File::open("in.rin").expect("file not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let lexer = RinLexer::new(&data);
    let tokens = lexer.tokenize().expect("Failed to tokenize rin");
    println!("{tokens:?}");

    let mut file = File::open("in.asm").expect("file not found");
    let mut abc = String::new();
    file.read_to_string(&mut abc).unwrap();
    let lexer = ASMLexer::new(&abc);
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
