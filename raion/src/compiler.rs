use std::{error::Error, fmt::Display};

use crate::{error::ErrorGenerator, token::Token, Location, WithLocation};
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
                if let Some(WithLocation {
                    value: token,
                    location,
                }) = token
                {
                    write!(
                        f,
                        "{}",
                        ErrorGenerator::new(
                            location,
                            format!("{style_bold}Unexpected token `{token}`{style_reset}"),
                            location.column.to_string().len()
                        )
                        .vertical_pipe(format!("{}", location.column))?
                        .write_line(location.column)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(location.row, "", '^', color_red)?
                        .build()
                    )
                } else {
                    write!(f, "Unexpected Token")
                }
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

    pub fn expect_token(&mut self, expected: T) -> Result<&Location, CompilerError<T>> {
        let token = self.peek(0).ok_or(CompilerError::UnexpectedToken(None))?;
        if *token.value() == expected {
            return Ok(&self.consume().unwrap().location);
        } else {
            return Err(CompilerError::UnexpectedToken(Some(token.clone())));
        }
    }
}
