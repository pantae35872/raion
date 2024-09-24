use std::{error::Error, fmt::Display, num::ParseIntError, path::Path};

use crate::{error::ErrorGenerator, token::Token, Location, WithLocation};
use inline_colorization::*;

pub mod asm_lexer;
pub mod rin_lexer;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String, Location),
    InvalidInterger(ParseIntError),
    InvalidEscapeSequence(String, Location),
    ExpectedEndDoubleQuote(Option<char>, Location),
    EndOfBuffer,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(buffer, location) => write!(
                f,
                "{}",
                ErrorGenerator::new(
                    location,
                    format!("{style_bold}Invalid token `{buffer}`{style_reset}"),
                    location.column.to_string().len()
                )
                .vertical_pipe(format!("{}", location.column))?
                .write_line(location.column)?
                .new_line()?
                .vertical_pipe("")?
                .pointer(location.row, "", '^', color_red)?
                .build()
            ),
            Self::InvalidInterger(e) => write!(f, "Trying to tokenize invalid number, {}", e),
            Self::InvalidEscapeSequence(escape_sequence, location) => write!(
                f,
                "{}",
                ErrorGenerator::new(
                    location,
                    format!(
                        "{style_bold}Invalid escape sequence `{escape_sequence}` {style_reset}"
                    ),
                    location.column.to_string().len(),
                )
                .vertical_pipe(format!("{}", location.column))?
                .write_line(location.column)?
                .new_line()?
                .vertical_pipe("")?
                .pointer(location.row, "", '^', color_red)?
                .build()
            ),
            Self::ExpectedEndDoubleQuote(_, location) => write!(
                f,
                "{}",
                ErrorGenerator::new(
                    location,
                    format!("{style_bold}Unclosed string literal {style_reset}"),
                    location.column.to_string().len(),
                )
                .vertical_pipe(format!("{}", location.column))?
                .write_line(location.column)?
                .new_line()?
                .vertical_pipe("")?
                .pointer(location.row, "", '^', color_red)?
                .build()
            ),
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

pub struct LexerBase<'a, T: Token> {
    buffer: &'a str,
    index: usize,
    row: usize,
    column: usize,
    tokens: Vec<WithLocation<T>>,
    file: &'a Path,
    saved_location: Location,
}

impl<'a, T: Token> LexerBase<'a, T> {
    pub fn new(buffer: &'a str, file: &'a Path) -> Self {
        Self {
            buffer,
            index: 0,
            row: 0,
            column: 1,
            tokens: Vec::new(),
            file,
            saved_location: Location::default(),
        }
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
            if byte as char == '\n' {
                self.row = 0;
                self.column += 1;
            }
            self.index += 1;
            self.row += 1;
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

    fn with_location(&self, token: T) -> WithLocation<T> {
        return WithLocation::new(token, self.current_location());
    }

    fn push(&mut self, token: T) {
        self.tokens.push(self.with_location(token));
    }

    fn consume_while<P, I>(&mut self, buffer: &mut String, predicate: P, ignored: I)
    where
        P: Fn(char) -> bool,
        I: Fn(char) -> bool,
    {
        while self.peek(0).is_some_and(|e| predicate(e)) {
            if self.peek(0).is_some_and(|e| ignored(e)) {
                self.consume();
                continue;
            }

            buffer.push(self.consume().unwrap());
        }
    }

    fn current_location(&self) -> Location {
        return self.saved_location.clone();
    }

    fn save_location(&mut self) {
        self.saved_location = Location::new(self.row, self.column, self.file.to_path_buf());
    }

    fn parse_interger(&mut self) -> Result<(), LexerError> {
        let mut buffer = String::new();
        self.save_location();
        if self.peek_match("0x") {
            self.consumes(2);

            self.consume_while(&mut buffer, |e| e.is_digit(16) || e == '_', |e| e == '_');

            self.push(Token::from_u64(
                u64::from_str_radix(&buffer, 16).map_err(LexerError::InvalidInterger)?,
            ));
        } else if self.peek_match("0o") {
            self.consumes(2);

            self.consume_while(&mut buffer, |e| e.is_digit(8) || e == '_', |e| e == '_');

            self.push(Token::from_u64(
                u64::from_str_radix(&buffer, 8).map_err(LexerError::InvalidInterger)?,
            ));
        } else if self.peek_match("0b") {
            self.consumes(2);

            self.consume_while(&mut buffer, |e| e.is_digit(2) || e == '_', |e| e == '_');

            self.push(Token::from_u64(
                u64::from_str_radix(&buffer, 2).map_err(LexerError::InvalidInterger)?,
            ));
        } else {
            buffer.push(self.consume().unwrap());

            self.consume_while(&mut buffer, |e| e.is_digit(10) || e == '_', |e| e == '_');

            self.push(Token::from_u64(
                buffer.parse::<u64>().map_err(LexerError::InvalidInterger)?,
            ));
        }
        return Ok(());
    }

    fn parse_string(&mut self) -> Result<(), LexerError> {
        let mut buffer = String::new();
        self.save_location();
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
                return Err(LexerError::ExpectedEndDoubleQuote(
                    Some('\n'),
                    self.current_location(),
                ));
            }

            if self.peek(0).is_some_and(|e| e == '\\') {
                let location = Location::new(self.row, self.column, self.file.to_path_buf());
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
                        return Err(LexerError::InvalidEscapeSequence(
                            format!("\\{}", invalid_escape),
                            location,
                        ));
                    }
                }
                continue;
            }
            if self.peek(0).is_some_and(|e| e == '\"') {
                self.consume();
            } else {
                return Err(LexerError::ExpectedEndDoubleQuote(
                    self.peek(0),
                    self.current_location(),
                ));
            }
            break;
        }
        self.push(Token::from_string(buffer));
        return Ok(());
    }
}
