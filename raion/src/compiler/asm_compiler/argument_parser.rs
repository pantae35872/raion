use xxhash_rust::xxh3::xxh3_64;

use crate::{
    compiler::CompilerBase,
    token::asm_token::{ASMToken, RegisterType},
};

use super::LabelReplace;

#[derive(Clone, Copy, Debug)]
pub enum ArgumentType {
    Register,
    RegisterSp,
    Number,
    Section,
    Label,
    DerefRegister,
}

pub struct ArgumentParser<'a> {
    compiler: &'a CompilerBase<ASMToken>,
    current_offset: usize,
    arguments_parse: Vec<ArgumentType>,
}

#[derive(Default)]
pub struct ParsedArgument {
    pub argument: Vec<u8>,
    pub label_replaces: Vec<LabelReplace>,
}

impl ParsedArgument {
    pub fn new(argument: Vec<u8>, label_replaces: Vec<LabelReplace>) -> Self {
        Self {
            argument,
            label_replaces,
        }
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
        self.compiler.peek(offset).is_some_and(expected)
    }

    pub fn is_valid(&mut self) -> bool {
        let len = self.arguments_parse.len();
        for (i, argument) in self.arguments_parse.iter().enumerate() {
            let valid = match argument {
                ArgumentType::Number => {
                    self.match_token(self.current_offset, |e| matches!(e, ASMToken::Number(_)))
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
                ArgumentType::RegisterSp => self.match_token(self.current_offset, |e| {
                    matches!(e, ASMToken::Register(RegisterType::SP))
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

    pub fn build(mut self) -> ParsedArgument {
        assert!(self.is_valid());
        let mut buffer = Vec::new();
        let mut label_replaces: Vec<LabelReplace> = Vec::new();
        let len = self.arguments_parse.len();
        for (i, argument) in self.arguments_parse.iter().enumerate() {
            match argument {
                ArgumentType::Register | ArgumentType::RegisterSp => {
                    let register = match self.compiler.peek(self.current_offset).unwrap() {
                        ASMToken::Register(register) => register,
                        _ => unreachable!(),
                    };
                    buffer.push(register.to_byte());
                }
                ArgumentType::Number => {
                    let number = match self.compiler.peek(self.current_offset).unwrap() {
                        ASMToken::Number(number) => number,
                        _ => unreachable!(),
                    };
                    buffer.extend_from_slice(&number.to_le_bytes());
                }
                ArgumentType::Section => {
                    let ident = match self.compiler.peek(self.current_offset).unwrap() {
                        ASMToken::Identifier(ident) => ident,
                        _ => unreachable!(),
                    };
                    buffer.extend_from_slice(&xxh3_64(ident.as_bytes()).to_le_bytes());
                }
                ArgumentType::Label => {
                    let ident = match self.compiler.peek(self.current_offset).unwrap() {
                        ASMToken::Identifier(ident) => ident,
                        _ => unreachable!(),
                    };
                    label_replaces.push(LabelReplace {
                        label: ident.clone(),
                        pos: buffer.len(),
                        line: self.compiler.current_line(),
                    });
                    buffer.extend_from_slice(&0u64.to_le_bytes());
                }
                ArgumentType::DerefRegister => {
                    let register = match self.compiler.peek(self.current_offset + 1).unwrap() {
                        ASMToken::Register(register) => register,
                        _ => unreachable!(),
                    };
                    buffer.push(register.to_byte());

                    self.current_offset += 2;
                }
            }
            self.current_offset += 2;
            if i == len - 1 {
                self.current_offset -= 1;
            }
        }
        return ParsedArgument::new(buffer, label_replaces);
    }
}
