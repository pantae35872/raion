use std::{path::Path, str::FromStr, sync::Arc};

use common::register::RegisterType;

use crate::{
    token::asm_token::{ASMKeyword, ASMToken, AttributeToken, InstructionType},
    WithLocation,
};

use super::{LexerBase, LexerError};

pub struct ASMLexer<'a> {
    base: LexerBase<'a, ASMToken>,
}

impl<'a> ASMLexer<'a> {
    pub fn new(buffer: &'a str, file: Arc<Path>) -> Self {
        Self {
            base: LexerBase::new(buffer, file),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<WithLocation<ASMToken>>, LexerError> {
        let mut buffer = String::new();

        while let Some(value) = self.base.peek(0) {
            if self.base.peek_match("//") {
                self.base
                    .consume_while(&mut String::new(), |e| e != '\n', |_| false);
                continue;
            }
            if value.is_alphabetic() {
                self.base
                    .consume_while(&mut buffer, |e| e.is_alphanumeric(), |_| false);

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

                if let Ok(keyword) = ASMKeyword::from_str(&buffer) {
                    self.base.push(ASMToken::Keyword(keyword));
                    buffer.clear();
                    continue;
                }

                self.base.push(ASMToken::Identifier(buffer.clone()));
                buffer.clear();
                continue;
            }
            if value == '<' {
                self.base.consume();
                self.base
                    .consume_while(&mut buffer, |e| e != '>', |_| false);
                self.base.push(ASMToken::HashName(buffer.clone()));
                if self.base.peek(0).is_some_and(|e| e == '>') {
                    self.base.consume();
                } else {
                    return Err(LexerError::ExpectedEndAngelBracket(
                        self.base.peek(0),
                        self.base.current_location(),
                    ));
                }
                buffer.clear();
                continue;
            }
            if value == '@' {
                self.base.consume();
                self.base
                    .consume_while(&mut buffer, |e| e.is_alphanumeric(), |_| false);
                self.base.push(ASMToken::Attribute(
                    AttributeToken::from_str(&buffer).map_err(|_| {
                        LexerError::InvalidAttribute(buffer.clone(), self.base.current_location())
                    })?,
                ));
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
            if value == '(' {
                self.base.consume();
                self.base.push(ASMToken::LRoundBracket);
                continue;
            }
            if value == ')' {
                self.base.consume();
                self.base.push(ASMToken::RRoundBracket);
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
            self.base.save_location();
            buffer.push(self.base.consume().unwrap());
            return Err(LexerError::InvalidToken(
                buffer,
                self.base.current_location(),
            ));
        }
        return Ok(self.base.tokens);
    }
}
