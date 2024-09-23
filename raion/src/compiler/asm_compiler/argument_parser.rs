use common::{inline_if, register::RegisterType};
use xxhash_rust::xxh3::xxh3_64;

use crate::{compiler::CompilerBase, token::asm_token::ASMToken, Location};

use super::LabelReplace;

#[derive(Clone, Copy, Debug)]
pub enum ArgumentType {
    Register,
    RegisterSp,
    U64,
    U32,
    Section,
    Label,
    DerefRegister,
    DerefRegisterOffset,
}

enum ParsedArgument {
    Register(RegisterType),
    U64(u64),
    U32(u32),
    Booolean(bool),
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
                ParsedArgument::Register(register) => buffer.push(register.to_byte()),
                ParsedArgument::Label(label) => {
                    label_replaces.push(LabelReplace {
                        label,
                        pos: buffer.len(),
                        location: location.clone(),
                    });
                    buffer.extend_from_slice(&0u64.to_le_bytes())
                }
                ParsedArgument::Booolean(value) => {
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
                ArgumentType::U64 | ArgumentType::U32 => {
                    self.match_token(self.current_offset, |e| matches!(e, ASMToken::Interger(_)))
                }
                ArgumentType::Register => {
                    self.match_token(self.current_offset, |e| e.is_register_and_general())
                }
                ArgumentType::Section | ArgumentType::Label => self
                    .match_token(self.current_offset, |e| {
                        matches!(e, ASMToken::Identifier(_))
                    }),
                ArgumentType::DerefRegister => {
                    let value = self
                        .match_token(self.current_offset, |e| matches!(e, ASMToken::LBracket))
                        && self.match_token(self.current_offset + 1, |e| {
                            matches!(e, ASMToken::Register(_))
                        })
                        && self.match_token(self.current_offset + 2, |e| {
                            matches!(e, ASMToken::RBracket)
                        });
                    self.current_offset += 2;
                    value
                }
                ArgumentType::DerefRegisterOffset => {
                    let value = self.match_token_sequence(&[
                        |e: &ASMToken| matches!(e, ASMToken::LBracket),
                        |e: &ASMToken| matches!(e, ASMToken::Register(_)),
                        |e: &ASMToken| matches!(e, ASMToken::Minus | ASMToken::Plus),
                        |e: &ASMToken| matches!(e, ASMToken::Interger(_)),
                        |e: &ASMToken| matches!(e, ASMToken::RBracket),
                    ]);
                    self.current_offset += 4;
                    value
                }
                ArgumentType::RegisterSp => self.match_token(self.current_offset, |e| {
                    matches!(e, ASMToken::Register(RegisterType::Sp))
                }),
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
                ArgumentType::Register | ArgumentType::RegisterSp => {
                    let register = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::Register(register) => register,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::Register(register.clone()));
                }
                ArgumentType::U64 => {
                    let number = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::Interger(number) => number,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::U64(*number));
                }
                ArgumentType::U32 => {
                    let number = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::Interger(number) => number,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::U32(*number as u32));
                }
                ArgumentType::Section => {
                    let ident = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::Identifier(ident) => ident,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::Section(xxh3_64(ident.as_bytes())));
                }
                ArgumentType::Label => {
                    let ident = match self.compiler.peek(self.current_offset).unwrap().value() {
                        ASMToken::Identifier(ident) => ident,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::Label(ident.clone()));
                }
                ArgumentType::DerefRegister => {
                    let register =
                        match self.compiler.peek(self.current_offset + 1).unwrap().value() {
                            ASMToken::Register(register) => register,
                            _ => unreachable!(),
                        };
                    arguments.push(ParsedArgument::Register(register.clone()));

                    self.current_offset += 2;
                }
                ArgumentType::DerefRegisterOffset => {
                    let register =
                        match self.compiler.peek(self.current_offset + 1).unwrap().value() {
                            ASMToken::Register(register) => register,
                            _ => unreachable!(),
                        };
                    let offset = match self.compiler.peek(self.current_offset + 3).unwrap().value()
                    {
                        ASMToken::Interger(offset) => offset,
                        _ => unreachable!(),
                    };

                    arguments.push(ParsedArgument::Register(register.clone()));
                    arguments.push(ParsedArgument::U32(*offset as u32));
                    let is_add = match self.compiler.peek(self.current_offset + 2).unwrap().value()
                    {
                        ASMToken::Plus => true,
                        ASMToken::Minus => false,
                        _ => unreachable!(),
                    };
                    arguments.push(ParsedArgument::Booolean(is_add));

                    self.current_offset += 4;
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
