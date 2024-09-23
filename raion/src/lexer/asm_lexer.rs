use std::{path::Path, str::FromStr};

use common::register::RegisterType;

use crate::{
    token::asm_token::{ASMToken, InstructionType},
    WithLocation,
};

use super::{LexerBase, LexerError};

pub struct ASMLexer<'a> {
    base: LexerBase<'a, ASMToken>,
}

impl<'a> ASMLexer<'a> {
    pub fn new(buffer: &'a str, file: &'a Path) -> Self {
        Self {
            base: LexerBase::new(buffer, file),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<WithLocation<ASMToken>>, LexerError> {
        let mut buffer = String::new();

        while let Some(value) = self.base.peek(0) {
            if value.is_alphabetic() {
                self.base.consume_while(
                    &mut buffer,
                    |e| e.is_alphanumeric() || e == '_' || e == '$',
                    |_| false,
                );

                if self.base.peek(0).is_some_and(|e| e == ':') {
                    self.base.consume();
                    self.base.push(ASMToken::Label(buffer.clone()));
                    buffer.clear();
                    continue;
                }

                if let Ok(instruction_type) = InstructionType::from_str(&buffer) {
                    self.base.push(ASMToken::Instruction(instruction_type));
                    buffer.clear();
                    continue;
                }

                if let Ok(register_type) = RegisterType::from_str(&buffer) {
                    self.base.push(ASMToken::Register(register_type));
                    buffer.clear();
                    continue;
                }

                self.base.push(ASMToken::Identifier(buffer.clone()));
                buffer.clear();
                continue;
            }
            if value.is_digit(10) {
                self.base.parse_interger()?;
                continue;
            }
            if value == ',' {
                self.base.consume();
                self.base.push(ASMToken::Comma);
                continue;
            }
            if value == '\n' {
                self.base.consume();
                self.base.push(ASMToken::NewLine);
                continue;
            }
            if value == ';' {
                self.base.consume();
                while self.base.peek(0).is_some_and(|e| e != '\n') {
                    self.base.consume();
                }
                continue;
            }
            if value == '+' {
                self.base.consume();
                self.base.push(ASMToken::Plus);
                continue;
            }
            if value == '[' {
                self.base.consume();
                self.base.push(ASMToken::LBracket);
                continue;
            }
            if value == ']' {
                self.base.consume();
                self.base.push(ASMToken::RBracket);
                continue;
            }
            if value == '{' {
                self.base.consume();
                self.base.push(ASMToken::LCurly);
                continue;
            }
            if value == '}' {
                self.base.consume();
                self.base.push(ASMToken::RCurly);
                continue;
            }
            if self.base.peek_match("->") {
                self.base.consumes(2);
                self.base.push(ASMToken::Arrow);
                continue;
            }
            if value == '-' {
                self.base.consume();
                self.base.push(ASMToken::Minus);
                continue;
            }
            if value == '\"' {
                self.base.parse_string()?;
                continue;
            }

            if value.is_whitespace() {
                self.base.consume();
                continue;
            }
            buffer.push(self.base.consume().unwrap());
            return Err(LexerError::InvalidToken(
                buffer,
                self.base.current_location(),
            ));
        }
        return Ok(self.base.tokens);
    }
}
