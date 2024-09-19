use std::{error::Error, fmt::Display};

use crate::token::Token;

pub mod asm_compiler;
pub mod rin_compiler;

#[derive(Debug)]
pub enum CompilerError<T: Token> {
    UnexpectedToken(Option<T>, usize),
    UndefinedLabel(String, usize),
    MultipleLabel(String, usize),
    InvalidArgument(usize),
}

impl<T: Token> Display for CompilerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token, line) => {
                write!(
                    f,
                    "Trying to compile with unexpected token `{token:?}` at line {line}",
                )
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
    tokens: Vec<T>,
    index: usize,
    line: usize,
}

impl<T: Token> CompilerBase<T> {
    pub fn new(tokens: Vec<T>) -> Self {
        Self {
            tokens,
            index: 0,
            line: 1,
        }
    }

    fn peek(&self, offset: usize) -> Option<&T> {
        return self.tokens.get(self.index + offset);
    }

    fn consume(&mut self) -> Option<&T> {
        if let Some(token) = self.tokens.get(self.index) {
            self.index += 1;
            if token.is_newline() {
                self.line += 1;
            }
            return Some(token);
        } else {
            return None;
        }
    }

    pub fn expect_token(&mut self, expected: T) -> Result<(), CompilerError<T>> {
        let token = self
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(None, self.line))?;
        if *token == expected {
            self.consume();
            return Ok(());
        } else {
            return Err(CompilerError::UnexpectedToken(
                Some(token.clone()),
                self.line,
            ));
        }
    }

    pub fn current_line(&self) -> usize {
        return self.line;
    }
}
