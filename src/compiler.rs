use std::{collections::HashMap, error::Error, fmt::Display};

use crate::token::{
    ASMToken, InstructionType, RegisterType, MOV_ADD2SP, MOV_NUM2REG, MOV_REG2MEM, MOV_REG2REG,
    MOV_REG2SP,
};

#[derive(Debug)]
pub enum CompilerError {
    UnexpectedToken(ASMToken),
    TokenNotFound,
    UndefinedLabel(String),
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(token) => {
                write!(f, "Trying to compile with unexpected token {}", token)
            }
            Self::TokenNotFound => {
                write!(f, "No token found where one was expected.")
            }
            Self::UndefinedLabel(label) => {
                write!(f, "Undefied label: {}", label)
            }
        }
    }
}

impl Error for CompilerError {}

pub struct ASMCompiler {
    tokens: Vec<ASMToken>,
    write_pos: usize,
    index: usize,
    labels: HashMap<String, usize>,
    label_replaces: Vec<(String, usize)>,
    byte_codes: Vec<u8>,
}

impl ASMCompiler {
    pub fn new(tokens: Vec<ASMToken>) -> Self {
        Self {
            tokens,
            write_pos: 0,
            index: 0,
            labels: HashMap::new(),
            label_replaces: Vec::new(),
            byte_codes: Vec::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.byte_codes.extend_from_slice(data);
        self.write_pos += data.len();
    }

    pub fn write_instruction(&mut self, opcode: u16, argument: &[u8]) {
        let opcode = opcode.to_le_bytes();
        let instruction_size = argument.len() + 3;
        self.write(&[instruction_size as u8, opcode[0], opcode[1]]);
        self.write(argument);
    }
    fn peek(&self, offset: usize) -> Option<&ASMToken> {
        return self.tokens.get(self.index + offset);
    }

    fn consume(&mut self) -> Option<&ASMToken> {
        if let Some(token) = self.tokens.get(self.index) {
            self.index += 1;
            return Some(token);
        } else {
            return None;
        }
    }

    fn compile_mov(&mut self, instruction_opcode: u16) -> Result<(), CompilerError> {
        match self.peek(1).ok_or(CompilerError::TokenNotFound)? {
            ASMToken::Register(register) => {
                let token = self.peek(2).ok_or(CompilerError::TokenNotFound)?;
                if *token != ASMToken::Comma {
                    return Err(CompilerError::UnexpectedToken(token.clone()));
                }

                match self.peek(3).ok_or(CompilerError::TokenNotFound)? {
                    ASMToken::Register(register1) => {
                        if *register == RegisterType::SP {
                            self.write_instruction(
                                instruction_opcode,
                                &[MOV_REG2SP, register1.to_byte()],
                            );
                        } else {
                            self.write_instruction(
                                instruction_opcode,
                                &[MOV_REG2REG, register.to_byte(), register1.to_byte()],
                            );
                        }
                    }
                    ASMToken::Number(number) => {
                        if *register == RegisterType::SP {
                            let mut arg = [MOV_ADD2SP].to_vec();
                            arg.extend_from_slice(&number.to_le_bytes());
                            self.write_instruction(instruction_opcode, &arg);
                        } else {
                            let mut arg = [MOV_NUM2REG].to_vec();
                            arg.push(register.to_byte());
                            arg.extend_from_slice(&number.to_le_bytes());
                            self.write_instruction(instruction_opcode, &arg);
                        }
                    }
                    token => return Err(CompilerError::UnexpectedToken(token.clone())),
                };
            }
            ASMToken::Number(address) => {
                let token = self.peek(2).ok_or(CompilerError::TokenNotFound)?;
                if *token != ASMToken::Comma {
                    return Err(CompilerError::UnexpectedToken(token.clone()));
                }

                let register1 = match self.peek(3).ok_or(CompilerError::TokenNotFound)? {
                    ASMToken::Register(reg) => reg,
                    token => return Err(CompilerError::UnexpectedToken(token.clone())),
                };
                let mut arg = [MOV_REG2MEM].to_vec();
                arg.extend_from_slice(&address.to_le_bytes());
                arg.push(register1.to_byte());
                self.write_instruction(instruction_opcode, &arg);
            }
            token => return Err(CompilerError::UnexpectedToken(token.clone())),
        };

        self.consume();
        self.consume();
        self.consume();
        self.consume();
        return Ok(());
    }

    fn replace_labels(&mut self) -> Result<(), CompilerError> {
        for (label, pos) in &self.label_replaces {
            let data = &mut self.byte_codes[*pos..(*pos + 8)];
            let label_data = self
                .labels
                .get(label)
                .ok_or(CompilerError::UndefinedLabel(label.clone()))?;
            data.copy_from_slice(&(*label_data as u64).to_le_bytes());
        }
        return Ok(());
    }

    pub fn compile(mut self) -> Result<Vec<u8>, CompilerError> {
        while let Some(token) = self.peek(0) {
            match token {
                ASMToken::Label(label) => {
                    self.labels.insert(label.clone(), self.write_pos);
                    self.consume();
                }
                ASMToken::Instruction(instruction) => match instruction {
                    InstructionType::Mov => self.compile_mov(instruction.opcode())?,
                    InstructionType::Inc | InstructionType::Pop | InstructionType::Push => {
                        let register = match self.peek(1).ok_or(CompilerError::TokenNotFound)? {
                            ASMToken::Register(reg) => reg,
                            token => return Err(CompilerError::UnexpectedToken(token.clone())),
                        };
                        self.write_instruction(instruction.opcode(), &[register.to_byte()]);
                        self.consume();
                        self.consume();
                    }
                    InstructionType::Cmp | InstructionType::Add | InstructionType::Sub => {
                        let register = match self.peek(1).ok_or(CompilerError::TokenNotFound)? {
                            ASMToken::Register(reg) => reg,
                            token => return Err(CompilerError::UnexpectedToken(token.clone())),
                        };
                        let token = self.peek(2).ok_or(CompilerError::TokenNotFound)?;
                        if *token != ASMToken::Comma {
                            return Err(CompilerError::UnexpectedToken(token.clone()));
                        }
                        let register1 = match self.peek(3).ok_or(CompilerError::TokenNotFound)? {
                            ASMToken::Register(reg) => reg,
                            token => return Err(CompilerError::UnexpectedToken(token.clone())),
                        };
                        self.write_instruction(
                            instruction.opcode(),
                            &[register.to_byte(), register1.to_byte()],
                        );
                        self.consume();
                        self.consume();
                        self.consume();
                        self.consume();
                    }
                    InstructionType::Jacn
                    | InstructionType::Jace
                    | InstructionType::Jacc
                    | InstructionType::Jacz => {
                        let register = match self.peek(1).ok_or(CompilerError::TokenNotFound)? {
                            ASMToken::Register(reg) => reg,
                            token => return Err(CompilerError::UnexpectedToken(token.clone())),
                        };
                        let token = self.peek(2).ok_or(CompilerError::TokenNotFound)?;
                        if *token != ASMToken::Comma {
                            return Err(CompilerError::UnexpectedToken(token.clone()));
                        }
                        let register1 = match self.peek(3).ok_or(CompilerError::TokenNotFound)? {
                            ASMToken::Register(reg) => reg,
                            token => return Err(CompilerError::UnexpectedToken(token.clone())),
                        };
                        let token = self.peek(4).ok_or(CompilerError::TokenNotFound)?;
                        if *token != ASMToken::Comma {
                            return Err(CompilerError::UnexpectedToken(token.clone()));
                        }
                        let mut arg = [register.to_byte(), register1.to_byte()].to_vec();
                        arg.extend_from_slice(&0u64.to_le_bytes());
                        self.write_instruction(instruction.opcode(), &arg);
                        let label = match self.peek(5).ok_or(CompilerError::TokenNotFound)? {
                            ASMToken::ToLabel(label) => label,
                            token => return Err(CompilerError::UnexpectedToken(token.clone())),
                        };
                        self.label_replaces
                            .push((label.clone(), self.write_pos - 8));
                        self.consume();
                        self.consume();
                        self.consume();
                        self.consume();
                        self.consume();
                        self.consume();
                    }
                    InstructionType::Jmp
                    | InstructionType::Jmc
                    | InstructionType::Jmz
                    | InstructionType::Jme
                    | InstructionType::Jmn => {
                        self.write_instruction(instruction.opcode(), &0u64.to_le_bytes());
                        let label = match self.peek(1).ok_or(CompilerError::TokenNotFound)? {
                            ASMToken::ToLabel(label) => label,
                            token => return Err(CompilerError::UnexpectedToken(token.clone())),
                        };
                        self.label_replaces
                            .push((label.clone(), self.write_pos - 8));
                        self.consume();
                        self.consume();
                    }
                    _ => {
                        self.write_instruction(instruction.opcode(), &[]);
                        self.consume();
                    }
                },
                _ => return Err(CompilerError::UnexpectedToken(token.clone())),
            }
            if self.peek(0).is_some_and(|e| *e == ASMToken::NewLine) {
                self.consume();
            } else {
                match self.peek(0) {
                    Some(token) => return Err(CompilerError::UnexpectedToken(token.clone())),
                    None => {}
                }
            }
        }
        self.replace_labels()?;
        return Ok(self.byte_codes);
    }
}
