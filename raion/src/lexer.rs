use std::{error::Error, fmt::Display, num::ParseIntError, str::FromStr};

use crate::token::asm_token::{ASMToken, InstructionType, RegisterType};

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String),
    InvalidNumber(ParseIntError),
    InvalidEscapeSequence(String),
    ExpectedEndDoubleQuote(Option<char>),
    EndOfBuffer,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(buffer) => write!(f, "Trying to tokenize invalid input: {}", buffer),
            Self::InvalidNumber(e) => write!(f, "Trying to tokenize invalid number, {}", e),
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
        return Self::InvalidNumber(value);
    }
}

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
                while self
                    .peek(0)
                    .is_some_and(|e| e.is_alphanumeric() || e == '_')
                {
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

                tokens.push(ASMToken::Identifier(buffer.clone()));
                buffer.clear();
                continue;
            }
            if value.is_digit(10) {
                buffer.push(self.consume().unwrap());
                if value == '0' && self.peek(0).is_some_and(|e| e == 'x') {
                    self.consume();
                    while self.peek(0).is_some_and(|e| e.is_digit(16)) {
                        buffer.push(self.consume().unwrap());
                    }
                    tokens.push(ASMToken::Number(u64::from_str_radix(&buffer, 16)?));
                    buffer.clear();
                    continue;
                }
                while self.peek(0).is_some_and(|e| e.is_digit(16)) {
                    buffer.push(self.consume().unwrap());
                }
                tokens.push(ASMToken::Number(buffer.parse::<u64>()?));
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
            if value == ';' {
                self.consume();
                while self.peek(0).is_some_and(|e| e != '\n') {
                    self.consume();
                }
                continue;
            }
            if value == '[' {
                self.consume();
                tokens.push(ASMToken::LBracket);
                continue;
            }
            if value == ']' {
                self.consume();
                tokens.push(ASMToken::RBracket);
                continue;
            }
            if value == '{' {
                self.consume();
                tokens.push(ASMToken::LCurly);
                continue;
            }
            if value == '}' {
                self.consume();
                tokens.push(ASMToken::RCurly);
                continue;
            }
            if value == '-' && self.peek(1).is_some_and(|e| e == '>') {
                self.consume();
                self.consume();
                tokens.push(ASMToken::Arrow);
                continue;
            }
            if value == '\"' {
                self.consume();
                loop {
                    while self
                        .peek(0)
                        .is_some_and(|e| e != '\\' && e != '\"' && e != '\n')
                    {
                        buffer.push(self.consume().unwrap());
                    }

                    if self.peek(0).is_some_and(|e| e == '\"')
                        && self.peek(1).is_some_and(|e| e == '\"')
                    {
                        self.consume();
                        self.consume();

                        if self.peek(0).is_some_and(|e| e == '\n') {
                            self.consume();
                        }
                        loop {
                            while self.peek(0).is_some_and(|e| e != '\\' && e != '\"') {
                                buffer.push(self.consume().unwrap());
                            }

                            if self.peek(0).is_some_and(|e| e == '\"')
                                && self.peek(1).is_some_and(|e| e == '\"')
                                && self.peek(2).is_some_and(|e| e == '\"')
                            {
                                if self.peek(3).is_some_and(|e| e == '\"') {
                                    buffer.push('\"');
                                    self.consume();
                                }
                                if self.peek(3).is_some_and(|e| e == '\"') {
                                    buffer.push('\"');
                                    self.consume();
                                }
                                self.consume();
                                self.consume();
                                self.consume();
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
                                        while self
                                            .peek(0)
                                            .is_some_and(|e| e.is_whitespace() || e == '\n')
                                        {
                                            self.consume();
                                        }
                                    }
                                }
                                continue;
                            }

                            if self.peek(0).is_some_and(|e| e == '\"')
                                || self.peek(1).is_some_and(|e| e == '\"')
                            {
                                if self.peek(0).is_some_and(|e| e == '\"')
                                    && self.peek(1).is_some_and(|e| e == '\"')
                                {
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

                tokens.push(ASMToken::String(buffer.clone()));
                buffer.clear();
                continue;
            }

            if value.is_whitespace() {
                self.consume();
                continue;
            }
            buffer.push(self.consume().unwrap());
            return Err(LexerError::InvalidToken(buffer));
        }
        return Ok(tokens);
    }
}
