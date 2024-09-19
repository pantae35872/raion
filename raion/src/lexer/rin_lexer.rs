use std::str::FromStr;

use crate::token::rin_token::{Keyword, Operator, PrimitiveType, RinToken};

use super::{LexerBase, LexerError};

pub struct RinLexer<'a> {
    base: LexerBase<'a>,
}

impl<'a> RinLexer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            base: LexerBase::new(buffer),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<RinToken>, LexerError> {
        let mut tokens = Vec::new();
        let mut buffer = String::new();

        while let Some(value) = self.base.peek(0) {
            if self.base.peek_match("//") {
                self.base.consume();
                while self.base.peek(0).is_some_and(|e| e != '\n') {
                    self.base.consume();
                }
                self.base.consume();
                continue;
            }
            if self.base.peek_match("->") {
                self.base.consumes(2);
                tokens.push(RinToken::Arrow);
                continue;
            }
            if value.is_alphabetic() {
                buffer.push(self.base.consume().unwrap());
                while self
                    .base
                    .peek(0)
                    .is_some_and(|e| e.is_alphanumeric() || e == '_')
                {
                    buffer.push(self.base.consume().unwrap());
                }

                if let Ok(keyword) = Keyword::from_str(&buffer) {
                    tokens.push(RinToken::Keyword(keyword));
                    buffer.clear();
                    continue;
                }

                if let Ok(primitive_type) = PrimitiveType::from_str(&buffer) {
                    tokens.push(RinToken::Type(primitive_type));
                    buffer.clear();
                    continue;
                }

                tokens.push(RinToken::Identifier(buffer.clone()));
                buffer.clear();
                continue;
            }
            if value.is_digit(10) {
                tokens.push(self.base.parse_interger()?);
                continue;
            }
            if value == ',' {
                self.base.consume();
                tokens.push(RinToken::Comma);
                continue;
            }
            if value == '\n' {
                self.base.consume();
                tokens.push(RinToken::NewLine);
                continue;
            }
            if value == ';' {
                self.base.consume();
                tokens.push(RinToken::Semicolon);
                continue;
            }
            if value == ':' {
                self.base.consume();
                tokens.push(RinToken::Colon);
                continue;
            }
            if value == '.' {
                self.base.consume();
                tokens.push(RinToken::Dot);
                continue;
            }
            if value == '+' || value == '-' || value == '*' || value == '/' {
                self.base.consume();
                tokens.push(RinToken::Operator(
                    Operator::from_str(&value.to_string()).unwrap(),
                ));
                continue;
            }
            if value == '=' {
                self.base.consume();
                tokens.push(RinToken::Equals);
                continue;
            }
            if value == '{' {
                self.base.consume();
                tokens.push(RinToken::LCurly);
                continue;
            }
            if value == '}' {
                self.base.consume();
                tokens.push(RinToken::RCurly);
                continue;
            }
            if value == '(' {
                self.base.consume();
                tokens.push(RinToken::LRoundBracket);
                continue;
            }
            if value == ')' {
                self.base.consume();
                tokens.push(RinToken::RRoundBracket);
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
