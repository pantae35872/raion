use std::{error::Error, fmt::Display, str::FromStr};

use crate::token::{ASMToken, InstructionType, RegisterType};

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(buffer) => write!(f, "Trying to tokenize invalid input: {}", buffer),
        }
    }
}

impl Error for LexerError {}

pub struct Lexer<'a> {
    buffer: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self { buffer, index: 0 }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        return self
            .buffer
            .as_bytes()
            .get(self.index + offset)
            .map(|&b| b as char);
    }

    fn consume(&mut self) -> Option<char> {
        if let Some(&byte) = self.buffer.as_bytes().get(self.index) {
            self.index += 1;
            return Some(byte as char);
        } else {
            return None;
        }
    }

    pub fn tokenize_asm(mut self) -> Result<Vec<ASMToken>, LexerError> {
        let mut tokens = Vec::new();
        let mut buffer = String::new();

        while let Some(value) = self.peek(0) {
            if value.is_alphabetic() {
                buffer.push(self.consume().unwrap());
                while self.peek(0).is_some_and(|e| e.is_alphanumeric()) {
                    buffer.push(self.consume().unwrap());
                }

                if self.peek(0).is_some_and(|e| e == ':') {
                    self.consume();
                    tokens.push(ASMToken::Label(buffer.clone()));
                    buffer.clear();
                    continue;
                }

                if let Ok(instruction_type) = InstructionType::from_str(&buffer) {
                    tokens.push(ASMToken::Instruction(instruction_type));
                    buffer.clear();
                    continue;
                }

                if let Ok(register_type) = RegisterType::from_str(&buffer) {
                    tokens.push(ASMToken::Register(register_type));
                    buffer.clear();
                    continue;
                }
            }
            if value.is_digit(10) {
                buffer.push(self.consume().unwrap());
                while self.peek(0).is_some_and(|e| e.is_digit(10)) {
                    buffer.push(self.consume().unwrap());
                }
                tokens.push(ASMToken::Number(buffer.parse::<u64>().unwrap()));
                buffer.clear();
                continue;
            }
            if value == ',' {
                self.consume();
                tokens.push(ASMToken::Comma);
                continue;
            }
            if value == '\n' {
                self.consume();
                tokens.push(ASMToken::NewLine);
                continue;
            }
            if value.is_whitespace() {
                self.consume();
                continue;
            }
            return Err(LexerError::InvalidToken(buffer));
        }
        return Ok(tokens);
    }
}
