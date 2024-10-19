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
    sin::sections::{
        Attribute, Attributes, Field, Interface, Procedure, SinSection, Structure, VProcedure,
    },
};
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    token::asm_token::{ASMKeyword, ASMToken, AttributeToken, InstructionType},
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
                        InstructionType::RetL
                        | InstructionType::Local
                        | InstructionType::LoadOs => self
                            .try_parse_argument(&[ArgumentType::U16])
                            .ok_or(CompilerError::InvalidArgument(location.clone()))?,
                        InstructionType::PushU64 => {
                            self.try_parse_argument(&[ArgumentType::U64])
                                .ok_or(CompilerError::InvalidArgument(location.clone()))?
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

    pub fn parse_hash(&mut self) -> Result<u64, CompilerError<ASMToken>> {
        let hash = match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(None))?
        {
            WithLocation {
                value: ASMToken::HashName(name),
                location: _,
            } => name,
            token => return Err(CompilerError::UnexpectedToken(Some(token.clone()))),
        }
        .clone();
        self.base.consume();
        return Ok(xxh3_64(hash.as_bytes()));
    }

    pub fn parse_attribute(&mut self) -> Result<Attribute, CompilerError<ASMToken>> {
        let attribute = match self
            .base
            .consume()
            .ok_or(CompilerError::UnexpectedToken(None))?
        {
            WithLocation {
                value: ASMToken::Attribute(attribute),
                ..
            } => attribute,
            token => return Err(CompilerError::UnexpectedToken(Some(token.clone()))),
        };
        match attribute {
            AttributeToken::Public => Ok(Attribute::Public),
            AttributeToken::Private => Ok(Attribute::Private),
            AttributeToken::Static => Ok(Attribute::Static),
            AttributeToken::Implemented => {
                self.base.expect_token(ASMToken::LRoundBracket)?;
                let hash = self.parse_hash()?;
                self.base.expect_token(ASMToken::RRoundBracket)?;
                Ok(Attribute::Implemented(hash))
            }
            AttributeToken::Contain => {
                self.base.expect_token(ASMToken::LRoundBracket)?;
                let hash = self.parse_hash()?;
                self.base.expect_token(ASMToken::RRoundBracket)?;
                Ok(Attribute::Contain(hash))
            }
            AttributeToken::Return => {
                self.base.expect_token(ASMToken::LRoundBracket)?;
                let hash = self.parse_hash()?;
                self.base.expect_token(ASMToken::RRoundBracket)?;
                Ok(Attribute::Return(hash))
            }
            AttributeToken::Overwrite => {
                self.base.expect_token(ASMToken::LRoundBracket)?;
                let hash = self.parse_hash()?;
                self.base.expect_token(ASMToken::RRoundBracket)?;
                Ok(Attribute::Overwrite(hash))
            }
            AttributeToken::Accept => {
                self.base.expect_token(ASMToken::LRoundBracket)?;
                let mut hashs = Vec::new();
                while let Some(token) = self.base.peek(0).cloned() {
                    match token {
                        WithLocation {
                            value: ASMToken::HashName(..),
                            ..
                        } => {
                            hashs.push(self.parse_hash()?);
                        }
                        WithLocation {
                            value: ASMToken::RRoundBracket,
                            ..
                        } => {
                            self.base.consume();
                            break;
                        }
                        unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
                    }
                    if matches!(
                        self.base
                            .peek(0)
                            .ok_or(CompilerError::UnexpectedToken(None))?,
                        WithLocation {
                            value: ASMToken::RRoundBracket,
                            ..
                        }
                    ) {
                        self.base.consume();
                        break;
                    }
                    self.base.expect_token(ASMToken::Comma)?;
                }
                Ok(Attribute::Accept(hashs))
            }
        }
    }

    pub fn parse_proc(&mut self) -> Result<Procedure, CompilerError<ASMToken>> {
        self.base
            .expect_token(ASMToken::Keyword(ASMKeyword::Proc))?;
        let procedure_hash = self.parse_hash()?;
        self.base.expect_token(ASMToken::Arrow)?;
        let (start, end) = self.parse_section(procedure_hash)?;
        let attr = self.attributes_buffer.clone();
        self.attributes_buffer.clear();
        Ok(Procedure::new(
            procedure_hash,
            start,
            end - start,
            Attributes::new(attr),
        ))
    }

    pub fn parse_interface(&mut self) -> Result<Interface, CompilerError<ASMToken>> {
        self.base
            .expect_token(ASMToken::Keyword(ASMKeyword::Interface))?;
        let interface_hash = self.parse_hash()?;
        let mut vprocs = Vec::new();
        self.base.expect_token(ASMToken::Arrow)?;
        self.base.expect_token(ASMToken::LBracket)?;
        let attr = self.attributes_buffer.clone();
        self.attributes_buffer.clear();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: ASMToken::Keyword(ASMKeyword::VProc),
                    ..
                } => {
                    self.base.consume();
                    let hash = self.parse_hash()?;
                    let attr = self.attributes_buffer.clone();
                    self.attributes_buffer.clear();
                    vprocs.push(VProcedure::new(hash, Attributes::new(attr)));
                }
                WithLocation {
                    value: ASMToken::Attribute(..),
                    ..
                } => {
                    let attribute = self.parse_attribute()?;
                    self.attributes_buffer.push(attribute);
                }
                WithLocation {
                    value: ASMToken::NewLine,
                    ..
                } => {
                    self.base.consume();
                }
                WithLocation {
                    value: ASMToken::RBracket,
                    ..
                } => {
                    break;
                }
                unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
            }
        }
        self.base.expect_token(ASMToken::RBracket)?;
        Ok(Interface::new(
            interface_hash,
            vprocs,
            Attributes::new(attr),
        ))
    }

    pub fn parse_struct(&mut self) -> Result<Structure, CompilerError<ASMToken>> {
        self.base
            .expect_token(ASMToken::Keyword(ASMKeyword::Struct))?;
        let interface_hash = self.parse_hash()?;
        let mut procedures = Vec::new();
        let mut fields = Vec::new();
        self.base.expect_token(ASMToken::Arrow)?;
        self.base.expect_token(ASMToken::LBracket)?;
        let attr = self.attributes_buffer.clone();
        self.attributes_buffer.clear();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: ASMToken::Keyword(ASMKeyword::Proc),
                    ..
                } => {
                    procedures.push(self.parse_proc()?);
                }
                WithLocation {
                    value: ASMToken::Keyword(ASMKeyword::Field),
                    ..
                } => {
                    self.base.consume();
                    let attr = self.attributes_buffer.clone();
                    self.attributes_buffer.clear();
                    let hash = self.parse_hash()?;
                    fields.push(Field::new(hash, Attributes::new(attr)));
                }
                WithLocation {
                    value: ASMToken::Attribute(..),
                    ..
                } => {
                    let attribute = self.parse_attribute()?;
                    self.attributes_buffer.push(attribute);
                }
                WithLocation {
                    value: ASMToken::NewLine,
                    ..
                } => {
                    self.base.consume();
                }
                WithLocation {
                    value: ASMToken::RBracket,
                    ..
                } => {
                    break;
                }
                unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
            }
        }
        self.base.expect_token(ASMToken::RBracket)?;
        Ok(Structure::new(
            interface_hash,
            fields,
            procedures,
            Attributes::new(attr),
        ))
    }

    pub fn compile(mut self) -> Result<(Vec<SinSection>, Vec<u8>), CompilerError<ASMToken>> {
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: ASMToken::Keyword(keyword),
                    ..
                } => match keyword {
                    ASMKeyword::Proc => {
                        let proc = self.parse_proc()?;
                        self.sections.push(SinSection::Procedure(proc));
                    }
                    ASMKeyword::Interface => {
                        let interface = self.parse_interface()?;
                        self.sections.push(SinSection::Interface(interface));
                    }
                    ASMKeyword::Struct => {
                        let structure = self.parse_struct()?;
                        self.sections.push(SinSection::Structure(structure));
                    }
                    _ => return Err(CompilerError::UnexpectedToken(Some(token))),
                },
                WithLocation {
                    value: ASMToken::Attribute(..),
                    ..
                } => {
                    let attribute = self.parse_attribute()?;
                    self.attributes_buffer.push(attribute);
                }
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
