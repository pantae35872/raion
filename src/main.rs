use lexer::Lexer;
use token::ASMToken;

pub mod lexer;
pub mod token;

fn main() {
    let lexer = Lexer::new("start:\nmov a8, 255\n");
    println!("{:?}", lexer.tokenize_asm());
}
