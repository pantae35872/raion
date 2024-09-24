use std::{path::Path, str::FromStr};

use crate::{
    token::rin_token::{Keyword, Operator, PrimitiveType, RinToken},
    WithLocation,
};

use super::{LexerBase, LexerError};

pub struct RinLexer<'a> {
    base: LexerBase<'a, RinToken>,
}

impl<'a> RinLexer<'a> {
    pub fn new(buffer: &'a str, file: &'a Path) -> Self {
        Self {
            base: LexerBase::new(buffer, file),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<WithLocation<RinToken>>, LexerError> {
        let mut buffer = String::new();

        while let Some(value) = self.base.peek(0) {
            if self.base.peek_match("//") {
                self.base
                    .consume_while(&mut String::new(), |e| e != '\n', |_| false);
                continue;
            }
            if self.base.peek_match("->") {
                self.base.save_location();
                self.base.consumes(2);
                self.base.push(RinToken::Arrow);
                continue;
            }
            if value.is_alphabetic() {
                self.base.save_location();
                self.base.consume_while(
                    &mut buffer,
                    |e| e.is_alphanumeric() || e == '_',
                    |_| false,
                );

                if let Ok(keyword) = Keyword::from_str(&buffer) {
                    self.base.push(RinToken::Keyword(keyword));
                    buffer.clear();
                    continue;
                }

                if let Ok(primitive_type) = PrimitiveType::from_str(&buffer) {
                    self.base.push(RinToken::Type(primitive_type));
                    buffer.clear();
                    continue;
                }

                self.base.push(RinToken::Identifier(buffer.clone()));
                buffer.clear();
                continue;
            }
            if value.is_digit(10) {
                self.base.parse_interger()?;
                continue;
            }
            if value == ',' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::Comma);
                continue;
            }
            if value == ';' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::Semicolon);
                continue;
            }
            if value == ':' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::Colon);
                continue;
            }
            if value == '.' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::Dot);
                continue;
            }
            if value == '+' || value == '-' || value == '*' || value == '/' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::Operator(
                    Operator::from_str(&value.to_string()).unwrap(),
                ));
                continue;
            }
            if value == '=' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::Equals);
                continue;
            }
            if value == '{' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::LCurly);
                continue;
            }
            if value == '}' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::RCurly);
                continue;
            }
            if value == '(' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::LRoundBracket);
                continue;
            }
            if value == ')' {
                self.base.save_location();
                self.base.consume();
                self.base.push(RinToken::RRoundBracket);
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
