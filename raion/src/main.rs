use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process::ExitCode,
};

use common::sin::{
    sections::{SectionType, SinSection},
    Sin,
};
use compiler::ASMCompiler;
use lexer::Lexer;

pub mod compiler;
pub mod lexer;
pub mod token;

fn main() -> ExitCode {
    let mut file = File::open("in.asm").expect("file not found");
    let mut abc = String::new();
    file.read_to_string(&mut abc).unwrap();
    let lexer = Lexer::new(&abc);
    let tokens = lexer.tokenize_asm().expect("Cannot parse");
    let compiler = ASMCompiler::new(tokens);
    todo!("Make this work with new syntax");
    //let data = [
    //    13, 16, 0, 7, 8, 17, 166, 232, 96, 28, 253, 105, 71, 6, 16, 0, 6, 1, 8, 4, 128, 0, 1, 3,
    //    0xFF, 0xFF, 0x69,
    //];
    //let sin = Sin::new(
    //    vec![
    //        SinSection::new(SectionType::Function, 12671084672237842783, 0, 25),
    //        SinSection::new(SectionType::Constant, 5145922347574273553, 26, 26),
    //    ],
    //    &data,
    //);

    //if Path::new("out.sin").exists() {
    //    fs::remove_file("out.sin").unwrap();
    //}
    //if let Ok(mut file) = File::create("out.sin") {
    //    file.write_all(&sin.to_bytes()).unwrap();
    //} else {
    //    eprintln!("Failed to create out.sin file");
    //    return ExitCode::FAILURE;
    //}
    /*match compiler.compile() {
        Ok((text, data)) => {
            let sin = Sin::new(&text, &data).to_bytes();

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
    };*/
    return ExitCode::SUCCESS;
}
