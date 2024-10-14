use std::collections::HashMap;

use argument_parser::{ArgumentParser, ArgumentType, ParsedArguments};
use common::{
    constants::{
        ADD_REG_W_NUM, ADD_REG_W_REG, ADD_SP_W_NUM, ARG_NUM, ARG_REG, MOV_ADD2SP,
        MOV_DEREF_REG2REG, MOV_DEREF_REG_WITH_OFFSET2REG, MOV_NUM2DEREF_REG_WITH_OFFSET,
        MOV_NUM2REG, MOV_REG2DEREF_REG_WITH_OFFSET, MOV_REG2REG, MOV_REG2SP,
        MOV_SECTION_ADDR2DEREF_REG_WITH_OFFSET, MOV_SECTION_ADDR_2REG, SUB_REG_W_NUM,
        SUB_REG_W_REG, SUB_SP_W_NUM,
    },
    sin::sections::{Attribute, Procedure, SinSection},
};
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    token::asm_token::{ASMToken, InstructionType},
    Location, WithLocation,
};

use super::{CompilerBase, CompilerError};

mod argument_parser;

#[derive(Clone, Debug)]
pub struct LabelReplace {
    label: String,
    pos: usize,
    location: Location,
}

pub struct ASMCompiler {
    base: CompilerBase<ASMToken>,
    data: Vec<u8>,
    sections: Vec<SinSection>,
    write_pos: usize,
    attributes_buffer: Vec<Attribute>,
}

impl ASMCompiler {
    pub fn new(tokens: Vec<WithLocation<ASMToken>>) -> Self {
        Self {
            base: CompilerBase::new(tokens),
            data: Vec::new(),
            sections: Vec::new(),
            write_pos: 0,
            attributes_buffer: Vec::new(),
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
                        label_replace.location.clone(),
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
            if matches!(token.value(), ASMToken::NewLine) {
                break;
            } else {
                self.base.consume();
            }
        }
    }

    pub fn parse_section(
        &mut self,
        procedure_hash: u64,
    ) -> Result<(u64, u64), CompilerError<ASMToken>> {
        let start = self.write_pos;
        self.base.expect_token(ASMToken::LCurly)?;
        let mut labels = HashMap::new();
        let mut label_replacess = Vec::new();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: ASMToken::Label(label),
                    location,
                } => {
                    if let Err(err) = labels.try_insert(label, self.write_pos - start) {
                        return Err(CompilerError::MultipleLabel(
                            err.entry.key().clone(),
                            location,
                        ));
                    }
                    self.base.consume();
                }
                WithLocation {
                    value: ASMToken::Instruction(instruction),
                    location,
                } => {
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
                                        ArgumentType::U64,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_ADD2SP;
                                    self.try_parse_argument(&[
                                        ArgumentType::RegisterSp,
                                        ArgumentType::U64,
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
                                .or_else(|| {
                                    subopcode = MOV_NUM2DEREF_REG_WITH_OFFSET;
                                    self.try_parse_argument(&[
                                        ArgumentType::DerefRegisterOffset,
                                        ArgumentType::U64,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_REG2DEREF_REG_WITH_OFFSET;
                                    self.try_parse_argument(&[
                                        ArgumentType::DerefRegisterOffset,
                                        ArgumentType::Register,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_DEREF_REG_WITH_OFFSET2REG;
                                    self.try_parse_argument(&[
                                        ArgumentType::Register,
                                        ArgumentType::DerefRegisterOffset,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = MOV_SECTION_ADDR2DEREF_REG_WITH_OFFSET;
                                    self.try_parse_argument(&[
                                        ArgumentType::DerefRegisterOffset,
                                        ArgumentType::Section,
                                    ])
                                })
                                .ok_or(CompilerError::InvalidArgument(location.clone()))?;
                            args.insert(0, vec![subopcode]);
                            args
                        }
                        InstructionType::Cmp | InstructionType::Mul | InstructionType::Div => self
                            .try_parse_argument(&[ArgumentType::Register, ArgumentType::Register])
                            .ok_or(CompilerError::InvalidArgument(location.clone()))?,
                        InstructionType::Add => {
                            let mut subopcode = ADD_REG_W_REG;
                            let mut args = self
                                .try_parse_argument(&[
                                    ArgumentType::Register,
                                    ArgumentType::Register,
                                ])
                                .or_else(|| {
                                    subopcode = ADD_REG_W_NUM;
                                    self.try_parse_argument(&[
                                        ArgumentType::Register,
                                        ArgumentType::U64,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = ADD_SP_W_NUM;
                                    self.try_parse_argument(&[
                                        ArgumentType::RegisterSp,
                                        ArgumentType::U64,
                                    ])
                                })
                                .ok_or(CompilerError::InvalidArgument(location.clone()))?;
                            args.insert(0, vec![subopcode]);
                            args
                        }
                        InstructionType::Sub => {
                            let mut subopcode = SUB_REG_W_REG;
                            let mut args = self
                                .try_parse_argument(&[
                                    ArgumentType::Register,
                                    ArgumentType::Register,
                                ])
                                .or_else(|| {
                                    subopcode = SUB_REG_W_NUM;
                                    self.try_parse_argument(&[
                                        ArgumentType::Register,
                                        ArgumentType::U64,
                                    ])
                                })
                                .or_else(|| {
                                    subopcode = SUB_SP_W_NUM;
                                    self.try_parse_argument(&[
                                        ArgumentType::RegisterSp,
                                        ArgumentType::U64,
                                    ])
                                })
                                .ok_or(CompilerError::InvalidArgument(location.clone()))?;
                            args.insert(0, vec![subopcode]);
                            args
                        }
                        InstructionType::Inc
                        | InstructionType::Pop
                        | InstructionType::Push
                        | InstructionType::Outc
                        | InstructionType::Savr
                        | InstructionType::Restr
                        | InstructionType::Exit => self
                            .try_parse_argument(&[ArgumentType::Register])
                            .ok_or(CompilerError::InvalidArgument(location.clone()))?,
                        InstructionType::Jmp
                        | InstructionType::Jmc
                        | InstructionType::Jmz
                        | InstructionType::Jme
                        | InstructionType::Jmn => {
                            let mut args = self
                                .try_parse_argument(&[ArgumentType::Label])
                                .ok_or(CompilerError::InvalidArgument(location.clone()))?;
                            args.insert(0, procedure_hash.to_le_bytes().to_vec());
                            args
                        }
                        InstructionType::Call => self
                            .try_parse_argument(&[ArgumentType::Section])
                            .ok_or(CompilerError::InvalidArgument(location.clone()))?,
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
                                .ok_or(CompilerError::InvalidArgument(location.clone()))?;
                            args.insert(2, procedure_hash.to_le_bytes().to_vec());
                            args
                        }
                        InstructionType::Enter => self
                            .try_parse_argument(&[ArgumentType::U64])
                            .ok_or(CompilerError::InvalidArgument(location.clone()))?,
                        InstructionType::Arg => {
                            let mut subopcode = ARG_NUM;
                            let mut args = self
                                .try_parse_argument(&[ArgumentType::U32, ArgumentType::U64])
                                .or_else(|| {
                                    subopcode = ARG_REG;
                                    self.try_parse_argument(&[
                                        ArgumentType::U32,
                                        ArgumentType::Register,
                                    ])
                                })
                                .ok_or(CompilerError::InvalidArgument(location.clone()))?;
                            args.insert(0, vec![subopcode]);
                            args
                        }
                        InstructionType::LArg => self
                            .try_parse_argument(&[ArgumentType::Register, ArgumentType::U32])
                            .ok_or(CompilerError::InvalidArgument(location.clone()))?,
                        InstructionType::Ret | InstructionType::Leave | InstructionType::Halt => {
                            ParsedArguments::default()
                        }
                    }
                    .finalize(location);
                    self.write_instruction(instruction.opcode(), &argument);

                    // Replace the label using the real offset not argument offset
                    for label_replace in label_replaces.iter_mut() {
                        label_replace.pos = self.write_pos - (argument.len() - label_replace.pos);
                    }
                    label_replacess.extend_from_slice(&label_replaces);
                    self.consume_until_newline();
                }
                WithLocation {
                    value: ASMToken::String(string),
                    ..
                } => {
                    self.write(string.as_bytes());
                    self.base.consume();
                }
                WithLocation {
                    value: ASMToken::NewLine,
                    ..
                } => {
                    self.base.consume();
                }
                WithLocation {
                    value: ASMToken::RCurly,
                    ..
                } => {
                    break;
                }
                unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
            }
        }
        self.replace_labels(label_replacess, labels)?;
        self.base.expect_token(ASMToken::RCurly)?;
        return Ok((start as u64, self.write_pos as u64));
    }

    pub fn parse_name(&mut self) -> Result<String, CompilerError<ASMToken>> {
        let name = match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(None))?
        {
            WithLocation {
                value: ASMToken::Identifier(ident),
                location: _,
            } => ident,
            token => return Err(CompilerError::UnexpectedToken(Some(token.clone()))),
        }
        .clone();
        self.base.consume();
        return Ok(name);
    }

    pub fn compile(mut self) -> Result<(Vec<SinSection>, Vec<u8>), CompilerError<ASMToken>> {
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: ASMToken::Identifier(ref ident),
                    ..
                } => match ident.as_str() {
                    "proc" => {
                        self.base.consume();
                        let procedure_name = self.parse_name()?;
                        let procedure_hash = xxh3_64(procedure_name.as_bytes());
                        self.base.expect_token(ASMToken::Arrow)?;
                        let (start, end) = self.parse_section(procedure_hash)?;
                        todo!();
                        //self.sections.push(SinSection::Procedure(Procedure::new(
                        //    procedure_name,
                        //    start,
                        //    end - start,
                        //)));
                    }
                    _ => return Err(CompilerError::UnexpectedToken(Some(token))),
                },
                WithLocation {
                    value: ASMToken::NewLine,
                    ..
                } => {
                    self.base.consume();
                }
                unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
            }
        }
        return Ok((self.sections, self.data));
    }
}
