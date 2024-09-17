use std::collections::HashMap;

use argument_parser::{ArgumentParser, ArgumentType, ParsedArguments};
use common::{
    constants::{
        MOV_ADD2SP, MOV_DEREF_REG2REG, MOV_NUM2REG, MOV_REG2REG, MOV_REG2SP, MOV_SECTION_ADDR_2REG,
    },
    sin::sections::{SectionType, SinSection},
};
use xxhash_rust::xxh3::xxh3_64;

use crate::token::asm_token::{ASMToken, InstructionType};

use super::{CompilerBase, CompilerError};

mod argument_parser;

#[derive(Default, Clone, Debug)]
pub struct LabelReplace {
    label: String,
    pos: usize,
    line: usize,
}

pub struct ASMCompiler {
    base: CompilerBase<ASMToken>,
    data: Vec<u8>,
    sections: Vec<SinSection>,
    write_pos: usize,
}

impl ASMCompiler {
    pub fn new(tokens: Vec<ASMToken>) -> Self {
        Self {
            base: CompilerBase::new(tokens),
            data: Vec::new(),
            sections: Vec::new(),
            write_pos: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
        self.write_pos += data.len();
    }

    pub fn write_instruction(&mut self, opcode: u16, argument: &[u8]) {
        let opcode = opcode.to_le_bytes();
        let instruction_size = argument.len() + 3;
        self.write(&[instruction_size as u8, opcode[0], opcode[1]]);
        self.write(argument);
    }

    fn replace_labels(
        &mut self,
        label_replaces: Vec<LabelReplace>,
        labels: HashMap<String, usize>,
    ) -> Result<(), CompilerError<ASMToken>> {
        for label_replace in &label_replaces {
            let data = &mut self.data[label_replace.pos..(label_replace.pos + 8)];
            let label_data =
                labels
                    .get(&label_replace.label)
                    .ok_or(CompilerError::UndefinedLabel(
                        label_replace.label.clone(),
                        label_replace.line,
                    ))?;
            data.copy_from_slice(&(*label_data as u64).to_le_bytes());
        }
        return Ok(());
    }

    fn try_parse_argument(&self, arg_types: &[ArgumentType]) -> Option<ParsedArguments> {
        let mut parser = ArgumentParser::new(&self.base);
        for arg_type in arg_types {
            parser = parser.parse(*arg_type);
        }
        if parser.is_valid() {
            Some(parser.build())
        } else {
            None
        }
    }

    fn consume_until_newline(&mut self) {
        while let Some(token) = self.base.peek(0) {
            if matches!(token, ASMToken::NewLine) {
                break;
            } else {
                self.base.consume();
            }
        }
    }

    pub fn parse_section(
        &mut self,
        function_hash: u64,
    ) -> Result<(u64, u64), CompilerError<ASMToken>> {
        let start = self.write_pos;
        self.base.expect_token(ASMToken::LCurly)?;
        let mut labels = HashMap::new();
        let mut label_replacess = Vec::new();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                ASMToken::Label(label) => {
                    if let Err(err) = labels.try_insert(label, self.write_pos - start) {
                        return Err(CompilerError::MultipleLabel(
                            err.entry.key().clone(),
                            self.base.current_line(),
                        ));
                    }
                    self.base.consume();
                }
                ASMToken::Instruction(instruction) => {
                    self.base.consume();
                    let (argument, mut label_replaces) = match instruction {
                        InstructionType::Mov => {
                            let mut subopcode = MOV_REG2REG;
                            let mut args = self
                                .try_parse_argument(&[
                                    ArgumentType::Register,
                                    ArgumentType::Register,
                                ])
                                .or_else(|| {
                                    subopcode = MOV_DEREF_REG2REG;
                                    self.try_parse_argument(&[
                                        ArgumentType::Register,
                                        ArgumentType::DerefRegister,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_NUM2REG;
                                    self.try_parse_argument(&[
                                        ArgumentType::Register,
                                        ArgumentType::Number,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_ADD2SP;
                                    self.try_parse_argument(&[
                                        ArgumentType::RegisterSp,
                                        ArgumentType::Number,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_REG2SP;
                                    self.try_parse_argument(&[
                                        ArgumentType::RegisterSp,
                                        ArgumentType::Register,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_SECTION_ADDR_2REG;
                                    self.try_parse_argument(&[
                                        ArgumentType::Register,
                                        ArgumentType::Section,
                                    ])
                                })
                                .ok_or(CompilerError::InvalidArgument(self.base.current_line()))?;
                            args.insert(0, vec![subopcode]);
                            args
                        }
                        InstructionType::Add | InstructionType::Sub | InstructionType::Cmp => self
                            .try_parse_argument(&[ArgumentType::Register, ArgumentType::Register])
                            .ok_or(CompilerError::InvalidArgument(self.base.current_line()))?,
                        InstructionType::Inc
                        | InstructionType::Pop
                        | InstructionType::Push
                        | InstructionType::Outc => self
                            .try_parse_argument(&[ArgumentType::Register])
                            .ok_or(CompilerError::InvalidArgument(self.base.current_line()))?,
                        InstructionType::Jmp
                        | InstructionType::Jmc
                        | InstructionType::Jmz
                        | InstructionType::Jme
                        | InstructionType::Jmn => {
                            let mut args = self
                                .try_parse_argument(&[ArgumentType::Label])
                                .ok_or(CompilerError::InvalidArgument(self.base.current_line()))?;
                            args.insert(0, function_hash.to_le_bytes().to_vec());
                            args
                        }
                        InstructionType::Call => self
                            .try_parse_argument(&[ArgumentType::Section])
                            .ok_or(CompilerError::InvalidArgument(self.base.current_line()))?,
                        InstructionType::Jacn
                        | InstructionType::Jacc
                        | InstructionType::Jace
                        | InstructionType::Jacz => {
                            let mut args = self
                                .try_parse_argument(&[
                                    ArgumentType::Register,
                                    ArgumentType::Register,
                                    ArgumentType::Label,
                                ])
                                .ok_or(CompilerError::InvalidArgument(self.base.current_line()))?;
                            args.insert(2, function_hash.to_le_bytes().to_vec());
                            args
                        }
                        InstructionType::Ret | InstructionType::Halt => ParsedArguments::default(),
                    }
                    .finalize();
                    self.write_instruction(instruction.opcode(), &argument);

                    for label_replace in label_replaces.iter_mut() {
                        label_replace.pos = self.write_pos - (argument.len() - label_replace.pos);
                    }
                    label_replacess.extend_from_slice(&label_replaces);
                    self.consume_until_newline();
                }
                ASMToken::String(string) => {
                    self.write(string.as_bytes());
                    self.base.consume();
                }
                ASMToken::NewLine => {
                    self.base.consume();
                }
                ASMToken::RCurly => {
                    break;
                }
                unexpected => {
                    return Err(CompilerError::UnexpectedToken(
                        Some(unexpected),
                        self.base.current_line(),
                    ))
                }
            }
        }
        self.replace_labels(label_replacess, labels)?;
        self.base.expect_token(ASMToken::RCurly)?;
        return Ok((start as u64, self.write_pos as u64 - 1));
    }

    pub fn parse_name(&mut self) -> Result<String, CompilerError<ASMToken>> {
        let function_name = match self.base.peek(0).ok_or(CompilerError::UnexpectedToken(
            None,
            self.base.current_line(),
        ))? {
            ASMToken::Identifier(ident) => ident,
            token => {
                return Err(CompilerError::UnexpectedToken(
                    Some(token.clone()),
                    self.base.current_line(),
                ))
            }
        }
        .clone();
        self.base.consume();
        return Ok(function_name);
    }

    pub fn compile(mut self) -> Result<(Vec<SinSection>, Vec<u8>), CompilerError<ASMToken>> {
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                ASMToken::Identifier(ref ident) => match ident.as_str() {
                    "fn" => {
                        self.base.consume();
                        let function_name = self.parse_name()?;
                        let function_hash = xxh3_64(function_name.as_bytes());
                        self.base.expect_token(ASMToken::Arrow)?;
                        let (start, end) = self.parse_section(function_hash)?;
                        self.sections.push(SinSection::new(
                            SectionType::Function,
                            function_hash,
                            start,
                            end,
                        ));
                    }
                    "const" => {
                        self.base.consume();
                        let const_name = self.parse_name()?;
                        let const_hash = xxh3_64(const_name.as_bytes());
                        self.base.expect_token(ASMToken::Arrow)?;
                        let (start, end) = self.parse_section(const_hash)?;
                        self.sections.push(SinSection::new(
                            SectionType::Constant,
                            xxh3_64(const_name.as_bytes()),
                            start,
                            end,
                        ));
                    }
                    _ => {
                        return Err(CompilerError::UnexpectedToken(
                            Some(token),
                            self.base.current_line(),
                        ))
                    }
                },
                ASMToken::NewLine => {
                    self.base.consume();
                }
                unexpected => {
                    return Err(CompilerError::UnexpectedToken(
                        Some(unexpected),
                        self.base.current_line(),
                    ))
                }
            }
        }
        return Ok((self.sections, self.data));
    }
}
