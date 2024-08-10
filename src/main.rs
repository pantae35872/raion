use std::{fs::File, io::Read};

use compiler::ASMCompiler;
use lexer::Lexer;

pub mod compiler;
pub mod lexer;
pub mod token;

fn main() {
    let mut file = File::open("in.asm").expect("file not found");
    let mut abc = String::new();
    file.read_to_string(&mut abc).unwrap();
    let lexer = Lexer::new(&abc);
    let compiler = ASMCompiler::new(lexer.tokenize_asm().expect("Cannot parse"));
    println!("{:?}", compiler.compile());
}
