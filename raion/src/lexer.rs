use std::{error::Error, fmt::Display, num::ParseIntError};

use crate::token::Token;

pub mod asm_lexer;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String),
    InvalidInterger(ParseIntError),
    InvalidEscapeSequence(String),
    ExpectedEndDoubleQuote(Option<char>),
    EndOfBuffer,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(buffer) => write!(f, "Trying to tokenize invalid input: {}", buffer),
            Self::InvalidInterger(e) => write!(f, "Trying to tokenize invalid number, {}", e),
            Self::InvalidEscapeSequence(escape_sequence) => {
                write!(f, "Invalid escape sequence: '{}'", escape_sequence)
            }
            Self::ExpectedEndDoubleQuote(found) => {
                write!(f, "Expected end double quote found: {:?}", found)
            }
            Self::EndOfBuffer => write!(
                f,
                "trying to read next token but already at the end of the buffer"
            ),
        }
    }
}

impl Error for LexerError {}

impl From<ParseIntError> for LexerError {
    fn from(value: ParseIntError) -> Self {
        return Self::InvalidInterger(value);
    }
}

pub struct LexerBase<'a> {
    buffer: &'a str,
    index: usize,
}

impl<'a> LexerBase<'a> {
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

    fn consumes(&mut self, count: usize) {
        for _ in 0..count {
            self.consume();
        }
    }

    fn peek_match(&self, matchs: &str) -> bool {
        let mut valid = true;
        for (index, charactor_match) in matchs.chars().enumerate() {
            if !self.peek(index).is_some_and(|e| e == charactor_match) {
                valid = false;
            }
        }
        return valid;
    }

    fn parse_interger<T: Token>(&mut self) -> Result<T, LexerError> {
        let mut buffer = String::new();
        if self.peek_match("0x") {
            self.consume();
            self.consume();

            while self.peek(0).is_some_and(|e| e.is_digit(16) || e == '_') {
                if self.peek(0).is_some_and(|e| e == '_') {
                    self.consume();
                    continue;
                }
                buffer.push(self.consume().unwrap());
            }

            return Ok(Token::from_u64(
                u64::from_str_radix(&buffer, 16).map_err(LexerError::InvalidInterger)?,
            ));
        }
        if self.peek_match("0o") {
            self.consume();
            self.consume();

            while self.peek(0).is_some_and(|e| e.is_digit(8)) {
                if self.peek(0).is_some_and(|e| e == '_') {
                    self.consume();
                    continue;
                }

                buffer.push(self.consume().unwrap());
            }

            return Ok(Token::from_u64(
                u64::from_str_radix(&buffer, 8).map_err(LexerError::InvalidInterger)?,
            ));
        }
        if self.peek_match("0b") {
            self.consumes(2);

            while self.peek(0).is_some_and(|e| e.is_digit(2)) {
                if self.peek(0).is_some_and(|e| e == '_') {
                    self.consume();
                    continue;
                }

                buffer.push(self.consume().unwrap());
            }

            return Ok(Token::from_u64(
                u64::from_str_radix(&buffer, 2).map_err(LexerError::InvalidInterger)?,
            ));
        }

        buffer.push(self.consume().unwrap());

        while self.peek(0).is_some_and(|e| e.is_digit(10) || e == '_') {
            if self.peek(0).is_some_and(|e| e == '_') {
                self.consume();
                continue;
            }

            buffer.push(self.consume().unwrap());
        }
        return Ok(Token::from_u64(
            buffer.parse::<u64>().map_err(LexerError::InvalidInterger)?,
        ));
    }

    fn parse_string<T: Token>(&mut self) -> Result<T, LexerError> {
        let mut buffer = String::new();
        self.consume();
        loop {
            while self
                .peek(0)
                .is_some_and(|e| e != '\\' && e != '\"' && e != '\n')
            {
                buffer.push(self.consume().unwrap());
            }

            if self.peek(0).is_some_and(|e| e == '\"') && self.peek(1).is_some_and(|e| e == '\"') {
                self.consumes(2);

                if self.peek(0).is_some_and(|e| e == '\n') {
                    self.consume();
                }
                loop {
                    while self.peek(0).is_some_and(|e| e != '\\' && e != '\"') {
                        buffer.push(self.consume().unwrap());
                    }

                    if self.peek_match("\"\"\"") {
                        if self.peek(3).is_some_and(|e| e == '\"') {
                            buffer.push('\"');
                            self.consume();
                        }
                        if self.peek(3).is_some_and(|e| e == '\"') {
                            buffer.push('\"');
                            self.consume();
                        }
                        self.consumes(3);
                        break;
                    }

                    if self.peek(0).is_some_and(|e| e == '\\') {
                        self.consume();
                        match self.consume().ok_or(LexerError::EndOfBuffer)? {
                            'b' => buffer.push('\u{08}'),
                            't' => buffer.push('\u{09}'),
                            'n' => buffer.push('\u{0A}'),
                            'f' => buffer.push('\u{0C}'),
                            'r' => buffer.push('\u{0D}'),
                            '\"' => buffer.push('\u{22}'),
                            '0' => buffer.push('\u{0}'),
                            '\\' => buffer.push('\u{5C}'),
                            _ => {
                                while self.peek(0).is_some_and(|e| e.is_whitespace() || e == '\n') {
                                    self.consume();
                                }
                            }
                        }
                        continue;
                    }

                    if self.peek(0).is_some_and(|e| e == '\"')
                        || self.peek(1).is_some_and(|e| e == '\"')
                    {
                        if self.peek_match("\"\"") {
                            buffer.push(self.consume().unwrap());
                        }
                        buffer.push(self.consume().unwrap());
                        continue;
                    }
                    break;
                }
                break;
            }

            if self.peek(0).is_some_and(|e| e == '\n') {
                return Err(LexerError::ExpectedEndDoubleQuote(Some('\n')));
            }

            if self.peek(0).is_some_and(|e| e == '\\') {
                self.consume();
                match self.consume().ok_or(LexerError::EndOfBuffer)? {
                    'b' => buffer.push('\u{08}'),
                    't' => buffer.push('\u{09}'),
                    'n' => buffer.push('\u{0A}'),
                    'f' => buffer.push('\u{0C}'),
                    'r' => buffer.push('\u{0D}'),
                    '"' => buffer.push('\u{22}'),
                    '0' => buffer.push('\u{0}'),
                    '\\' => buffer.push('\u{5C}'),
                    invalid_escape => {
                        return Err(LexerError::InvalidEscapeSequence(format!(
                            "\\{}",
                            invalid_escape
                        )))
                    }
                }
                continue;
            }
            if self.peek(0).is_some_and(|e| e == '\"') {
                self.consume();
            } else {
                return Err(LexerError::ExpectedEndDoubleQuote(self.peek(0)));
            }
            break;
        }
        return Ok(Token::from_string(buffer));
    }
}
