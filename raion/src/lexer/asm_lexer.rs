use std::str::FromStr;

use crate::token::asm_token::{ASMToken, InstructionType, RegisterType};

use super::{LexerBase, LexerError};

pub struct ASMLexer<'a> {
    base: LexerBase<'a>,
}

impl<'a> ASMLexer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            base: LexerBase::new(buffer),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<ASMToken>, LexerError> {
        let mut tokens = Vec::new();
        let mut buffer = String::new();

        while let Some(value) = self.base.peek(0) {
            if value.is_alphabetic() {
                buffer.push(self.base.consume().unwrap());
                while self
                    .base
                    .peek(0)
                    .is_some_and(|e| e.is_alphanumeric() || e == '_')
                {
                    buffer.push(self.base.consume().unwrap());
                }

                if self.base.peek(0).is_some_and(|e| e == ':') {
                    self.base.consume();
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

                tokens.push(ASMToken::Identifier(buffer.clone()));
                buffer.clear();
                continue;
            }
            if value.is_digit(10) {
                tokens.push(self.base.parse_interger()?);
                continue;
            }
            if value == ',' {
                self.base.consume();
                tokens.push(ASMToken::Comma);
                continue;
            }
            if value == '\n' {
                self.base.consume();
                tokens.push(ASMToken::NewLine);
                continue;
            }
            if value == ';' {
                self.base.consume();
                while self.base.peek(0).is_some_and(|e| e != '\n') {
                    self.base.consume();
                }
                continue;
            }
            if value == '[' {
                self.base.consume();
                tokens.push(ASMToken::LBracket);
                continue;
            }
            if value == ']' {
                self.base.consume();
                tokens.push(ASMToken::RBracket);
                continue;
            }
            if value == '{' {
                self.base.consume();
                tokens.push(ASMToken::LCurly);
                continue;
            }
            if value == '}' {
                self.base.consume();
                tokens.push(ASMToken::RCurly);
                continue;
            }
            if self.base.peek_match("->") {
                self.base.consumes(2);
                tokens.push(ASMToken::Arrow);
                continue;
            }
            if value == '\"' {
                tokens.push(self.base.parse_string()?);
                continue;
            }

            if value.is_whitespace() {
                self.base.consume();
                continue;
            }
            buffer.push(self.base.consume().unwrap());
            return Err(LexerError::InvalidToken(buffer));
        }
        return Ok(tokens);
    }
}
