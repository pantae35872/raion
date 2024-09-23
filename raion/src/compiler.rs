use std::{error::Error, fmt::Display, fs::File, io::Read};

use crate::{token::Token, Location, WithLocation};
use inline_colorization::*;

pub mod asm_compiler;
pub mod rin_compiler;

#[derive(Debug)]
pub enum CompilerError<T: Token> {
    UnexpectedToken(Option<WithLocation<T>>),
    UndefinedLabel(String, Location),
    MultipleLabel(String, Location),
    InvalidArgument(Location),
}

impl<T: Token> Display for CompilerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token) => {
                write!(
                    f,
                    "{style_bold}Trying to compile with unexpected token `{}`{style_reset}{}",
                    token
                        .clone()
                        .map(|e| format!("{}", e.value()))
                        .unwrap_or("".to_string()),
                    token
                        .clone()
                        .map(|e| format!(
                            " \n {color_blue}{style_bold}---->{style_reset}{color_reset} {}\n",
                            e.location()
                        ))
                        .unwrap_or("".to_string())
                )?;

                if let Some(token) = token {
                    let mut file =
                        File::open(token.location.file()).expect("Compilation file went missing");
                    let mut file_buf = String::new();
                    file.read_to_string(&mut file_buf).unwrap();
                    let line = file_buf.lines().nth(token.location.column() - 1).unwrap();
                    write!(
                        f,
                        "{color_blue}{style_bold}{} |{style_reset}{color_reset} {line}\n",
                        token.location.column()
                    )?;
                    for _ in 0..token.location.column().to_string().len() {
                        write!(f, " ")?;
                    }
                    write!(f, " {color_blue}{style_bold}|{style_reset}{color_reset} ")?;
                    for _ in 0..token.location.row - 1 {
                        write!(f, " ")?;
                    }
                    write!(f, "{color_yellow}{style_bold}^{style_reset}{color_reset}")?;
                }
                return Ok(());
            }
            Self::UndefinedLabel(label, line) => {
                write!(f, "Undefied label `{label}` at line {line}")
            }
            Self::MultipleLabel(label, line) => {
                write!(
                    f,
                    "the label name `{label}` is defined multiple times at line {line}",
                )
            }
            Self::InvalidArgument(line) => {
                write!(f, "invalid argument on line {line}")
            }
        }
    }
}

impl<T: Token> Error for CompilerError<T> {}

pub struct CompilerBase<T: Token> {
    tokens: Vec<WithLocation<T>>,
    index: usize,
}

impl<T: Token> CompilerBase<T> {
    pub fn new(tokens: Vec<WithLocation<T>>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self, offset: usize) -> Option<&WithLocation<T>> {
        return self.tokens.get(self.index + offset);
    }

    fn consume(&mut self) -> Option<&WithLocation<T>> {
        if let Some(token) = self.tokens.get(self.index) {
            self.index += 1;
            return Some(token);
        } else {
            return None;
        }
    }

    pub fn expect_token(&mut self, expected: T) -> Result<(), CompilerError<T>> {
        let token = self.peek(0).ok_or(CompilerError::UnexpectedToken(None))?;
        if *token.value() == expected {
            self.consume();
            return Ok(());
        } else {
            return Err(CompilerError::UnexpectedToken(Some(token.clone())));
        }
    }
}
