use common::inline_if;
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    compiler::CompilerBase,
    token::asm_token::{ASMToken, IntergerType},
    Location,
};

use super::LabelReplace;

#[derive(Clone, Copy, Debug)]
pub enum ArgumentType {
    U64,
    U32,
    U16,
    U8,
    I8,
    I16,
    I32,
    I64,
    Section,
    Label,
}

enum ParsedArgument {
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Boolean(bool),
    Section(u64),
    Label(String),
    Buffer(Vec<u8>),
}

pub struct ArgumentParser<'a> {
    compiler: &'a CompilerBase<ASMToken>,
    current_offset: usize,
    arguments_parse: Vec<ArgumentType>,
}

#[derive(Default)]
pub struct ParsedArguments {
    arguments: Vec<ParsedArgument>,
}

impl ParsedArguments {
    fn new(arguments: Vec<ParsedArgument>) -> Self {
        Self { arguments }
    }

    pub fn insert(&mut self, index: usize, buffer: Vec<u8>) {
        self.arguments.insert(index, ParsedArgument::Buffer(buffer));
    }

    pub fn finalize(self, location: Location) -> (Vec<u8>, Vec<LabelReplace>) {
        let mut label_replaces = Vec::new();
        let mut buffer = Vec::new();

        for arg in self.arguments {
            match arg {
                ParsedArgument::U64(data) | ParsedArgument::Section(data) => {
                    buffer.extend_from_slice(&data.to_le_bytes())
                }
                ParsedArgument::U32(value) => buffer.extend_from_slice(&value.to_le_bytes()),
                ParsedArgument::U16(value) => buffer.extend_from_slice(&value.to_le_bytes()),
                ParsedArgument::U8(value) => buffer.extend_from_slice(&value.to_le_bytes()),
                ParsedArgument::I8(value) => buffer.extend_from_slice(&value.to_le_bytes()),
                ParsedArgument::I16(value) => buffer.extend_from_slice(&value.to_le_bytes()),
                ParsedArgument::I32(value) => buffer.extend_from_slice(&value.to_le_bytes()),
                ParsedArgument::I64(value) => buffer.extend_from_slice(&value.to_le_bytes()),
                ParsedArgument::Label(label) => {
                    label_replaces.push(LabelReplace {
                        label,
                        pos: buffer.len(),
                        location: location.clone(),
                    });
                    buffer.extend_from_slice(&0u64.to_le_bytes())
                }
                ParsedArgument::Boolean(value) => {
                    buffer.push(inline_if!(value, 1, 0));
                }
                ParsedArgument::Buffer(data) => buffer.extend_from_slice(&data),
            }
        }

        return (buffer, label_replaces);
    }
}

impl<'a> ArgumentParser<'a> {
    pub fn new(compiler: &'a CompilerBase<ASMToken>) -> Self {
        Self {
            compiler,
            current_offset: 0,
            arguments_parse: Vec::new(),
        }
    }

    pub fn parse(mut self, argument_type: ArgumentType) -> Self {
        self.arguments_parse.push(argument_type);
        return self;
    }

    fn match_token(&self, offset: usize, expected: impl Fn(&ASMToken) -> bool) -> bool {
        self.compiler
            .peek(offset)
            .is_some_and(|e| expected(e.value()))
    }

    fn match_token_sequence(&self, expected: &[for<'b> fn(&'b ASMToken) -> bool]) -> bool {
        expected
            .iter()
            .enumerate()
            .all(|(i, expected)| self.match_token(self.current_offset + i, expected))
    }

    pub fn is_valid(&mut self) -> bool {
        let len = self.arguments_parse.len();
        for (i, argument) in self.arguments_parse.iter().enumerate() {
            let valid = match argument {
                ArgumentType::I64 | ArgumentType::I32 | ArgumentType::I16 | ArgumentType::I8 => {
                    todo!("support for negative number")
                }
                ArgumentType::U64 | ArgumentType::U32 | ArgumentType::U16 | ArgumentType::U8 => {
                    let value = self
                        .match_token(self.current_offset, |e| matches!(e, ASMToken::Interger(_)))
                        && self.match_token(self.current_offset + 1, |e| {
                            let v = match argument {
                                ArgumentType::U64 => IntergerType::U64,
                                ArgumentType::U32 => IntergerType::U32,
                                ArgumentType::U16 => IntergerType::U16,
                                ArgumentType::U8 => IntergerType::U8,
                                _ => unreachable!(),
                            };
                            match e {
                                ASMToken::IntergerType(e) => v == *e,
                                _ => false,
                            }
                        });
                    self.current_offset += 1;
                    value
                }
                ArgumentType::Label => {
                    self.match_token(self.current_offset, |e| matches!(e, ASMToken::Interger(_)))
                }
                ArgumentType::Section => {
                    self.match_token(self.current_offset, |e| matches!(e, ASMToken::HashName(_)))
                }
            };

            if !valid {
                return false;
            }

            self.current_offset += 1;

            if i != len - 1 {
                if !self.match_token(self.current_offset, |e| matches!(e, ASMToken::Comma)) {
                    return false;
                }

                self.current_offset += 1;
            }
        }
        self.current_offset = 0;
        return true;
    }

    pub fn build(mut self) -> ParsedArguments {
        assert!(self.is_valid());
        let mut arguments = Vec::new();
        let len = self.arguments_parse.len();
        for (i, argument) in self.arguments_parse.iter().enumerate() {
            match argument {
                ArgumentType::U64 | ArgumentType::U32 | ArgumentType::U16 | ArgumentType::U8 => {
                    let number = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::Interger(number) => number,
                        _ => unreachable!(),
                    };
                    // Skip the type because we already verify
                    self.current_offset += 1;
                    arguments.push(match argument {
                        ArgumentType::U64 => ParsedArgument::U64(*number),
                        ArgumentType::U32 => ParsedArgument::U32(*number as u32),
                        ArgumentType::U16 => ParsedArgument::U16(*number as u16),
                        ArgumentType::U8 => ParsedArgument::U8(*number as u8),
                        _ => unreachable!(),
                    });
                }
                ArgumentType::I64 | ArgumentType::I32 | ArgumentType::I16 | ArgumentType::I8 => {
                    todo!("support for negative interger")
                }
                ArgumentType::Section => {
                    let hash_name = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::HashName(hash_name) => hash_name,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::Section(xxh3_64(hash_name.as_bytes())));
                }
                ArgumentType::Label => {
                    let ident = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::Identifier(ident) => ident,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::Label(ident.clone()));
                }
            }
            self.current_offset += 2;
            if i == len - 1 {
                self.current_offset -= 1;
            }
        }
        return ParsedArguments::new(arguments);
    }
}
