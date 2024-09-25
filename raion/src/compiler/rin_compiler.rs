use std::fmt::Display;

use generator::{Generator, GeneratorError};

use crate::{
    token::rin_token::{Keyword, Operator, PrimitiveType, RinToken},
    WithLocation,
};

use super::{CompilerBase, CompilerError};

mod generator;

#[derive(Default, Debug)]
pub struct RinAst {
    imports: Vec<Import>,
    module: Vec<Module>,
    procedures: Vec<WithLocation<Procedure>>,
}

#[derive(Debug)]
struct Import {
    path: Path,
}

#[derive(Debug)]
struct Module {
    name: String,
}

#[derive(Debug)]
struct Procedure {
    name: WithLocation<String>,
    return_type: WithLocation<Type>,
    parameters: Vec<Parameter>,
    body: WithLocation<Block>,
}

#[derive(Debug)]
struct Block {
    body: Vec<WithLocation<Statement>>,
}

#[derive(Debug)]
struct Parameter {
    param_type: WithLocation<Type>,
    name: WithLocation<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Type {
    U64,
    U32,
    U16,
    U8,
    I8,
    I16,
    I32,
    I64,
    Bool,
    Void,
    //Struct(String)
}

pub enum TypeSizes {
    SizeVoid,
    SizeU8,
    SizeU16,
    SizeU32,
    SizeU64,
}

#[derive(Debug)]
enum Statement {
    Expression(WithLocation<Expression>),
    Return(WithLocation<Expression>),
    VariableDecl {
        name: WithLocation<String>,
        value: WithLocation<Expression>,
    },
    VariableMutate {
        name: WithLocation<String>,
        value: WithLocation<Expression>,
    },
}

#[derive(Debug)]
enum Expression {
    Term(WithLocation<Term>),
    BinaryOp(
        Box<WithLocation<Expression>>,
        WithLocation<BinaryOperator>,
        Box<WithLocation<Expression>>,
    ),
}

#[derive(Debug)]
enum Term {
    Literal(WithLocation<Literal>),
    ProcedureCall(WithLocation<Path>, Vec<WithLocation<Expression>>),
    LocalVariableAccess(WithLocation<String>),
}

#[derive(Debug)]
enum Literal {
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    path: Vec<String>,
}

pub struct RinCompiler<'a> {
    base: CompilerBase<RinToken>,
    program: RinAst,
    module_path: &'a Path,
}

impl<'a> RinCompiler<'a> {
    pub fn new(tokens: Vec<WithLocation<RinToken>>, module_path: &'a Path) -> Self {
        Self {
            base: CompilerBase::new(tokens),
            program: RinAst::default(),
            module_path,
        }
    }

    pub fn program(&self) -> &RinAst {
        return &self.program;
    }

    pub fn generate(&self) -> Result<String, GeneratorError> {
        let generator = Generator::new();
        return generator.generate(&self.program, self.module_path);
    }

    pub fn parse(&mut self) -> Result<(), CompilerError<RinToken>> {
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: RinToken::Keyword(keyword),
                    location,
                } => match keyword {
                    Keyword::Import => {
                        let import = self.parse_imports()?;
                        self.program.imports.push(import);
                    }
                    Keyword::Module => {
                        let module = self.parse_module()?;
                        self.program.module.push(module);
                    }
                    Keyword::Procedure => {
                        let proc = self.parse_proc()?;
                        self.program
                            .procedures
                            .push(WithLocation::new(proc, location));
                    }
                    unexpected => {
                        return Err(CompilerError::UnexpectedToken(Some(WithLocation::new(
                            RinToken::Keyword(unexpected),
                            location,
                        ))));
                    }
                },
                unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
            }
        }
        return Ok(());
    }

    fn parse_proc(&mut self) -> Result<Procedure, CompilerError<RinToken>> {
        self.base
            .expect_token(RinToken::Keyword(Keyword::Procedure))?;
        let name = self.parse_identifier()?;
        let params = self.parse_parameters()?;
        self.base.expect_token(RinToken::Colon)?;
        let return_type = self.parse_type()?;
        self.base.expect_token(RinToken::Arrow)?;
        let block = self.parse_block()?;
        return Ok(Procedure {
            name,
            return_type,
            parameters: params,
            body: block,
        });
    }

    fn parse_block(&mut self) -> Result<WithLocation<Block>, CompilerError<RinToken>> {
        let loc = self.base.expect_token(RinToken::LCurly)?.clone();
        let mut statements = Vec::new();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: RinToken::Keyword(_) | RinToken::Identifier(_) | RinToken::Type(_),
                    location,
                } => {
                    let statement = self.parse_statement()?;
                    statements.push(WithLocation::new(statement, location));
                }
                WithLocation {
                    value: RinToken::RCurly,
                    ..
                } => {
                    break;
                }
                unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
            }
        }
        self.base.expect_token(RinToken::RCurly)?;
        return Ok(WithLocation::new(Block { body: statements }, loc));
    }

    fn parse_statement(&mut self) -> Result<Statement, CompilerError<RinToken>> {
        let value = match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(None))?
            .clone()
        {
            WithLocation {
                value: RinToken::Keyword(Keyword::Let),
                ..
            } => {
                self.base.consume();
                let name = self.parse_identifier()?;
                self.base.expect_token(RinToken::Equals)?;
                let value = self.parse_expression(0)?;
                Ok(Statement::VariableDecl { name, value })
            }
            WithLocation {
                value: RinToken::Identifier(_),
                ..
            } => {
                if self.base.peek(1).is_some_and(|e| {
                    *e.value() == RinToken::Dot || *e.value() == RinToken::LRoundBracket
                }) {
                    let expr = self.parse_expression(0)?;
                    Ok(Statement::Expression(expr))
                } else {
                    let name = self.parse_identifier()?;
                    self.base.expect_token(RinToken::Equals)?;
                    let value = self.parse_expression(0)?;
                    Ok(Statement::VariableMutate { name, value })
                }
            }
            WithLocation {
                value: RinToken::Keyword(keyword),
                location,
            } => match keyword {
                Keyword::Return => {
                    self.base.consume();
                    let expr = self.parse_expression(0)?;
                    Ok(Statement::Return(expr))
                }
                unexpected => Err(CompilerError::UnexpectedToken(Some(WithLocation::new(
                    RinToken::Keyword(unexpected),
                    location,
                )))),
            },
            _ => Ok(Statement::Expression(self.parse_expression(0)?)),
        };

        self.base.expect_token(RinToken::Semicolon)?;
        return value;
    }

    fn parse_expression(
        &mut self,
        min_prec: usize,
    ) -> Result<WithLocation<Expression>, CompilerError<RinToken>> {
        let WithLocation {
            value: lhs,
            location,
        } = self.parse_term()?;
        let mut lhs = WithLocation::new(
            Expression::Term(WithLocation::new(lhs, location.clone())),
            location.clone(),
        );
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: RinToken::Operator(operator),
                    location,
                } => {
                    if operator.prec() < min_prec {
                        break;
                    }
                    self.base.consume();
                    let rhs = self.parse_expression(operator.prec() + 1)?;

                    lhs = WithLocation::new(
                        Expression::BinaryOp(
                            lhs.into(),
                            WithLocation::new(operator.into(), location.clone()),
                            rhs.into(),
                        ),
                        location.clone(),
                    );
                }
                _ => break,
            }
        }
        return Ok(lhs);
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, CompilerError<RinToken>> {
        self.base.expect_token(RinToken::LRoundBracket)?;
        let mut params = Vec::new();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: RinToken::Type(_),
                    ..
                } => {
                    let param = self.parse_parameter()?;
                    params.push(param);
                }
                WithLocation {
                    value: RinToken::RRoundBracket,
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
                    .ok_or(CompilerError::UnexpectedToken(None,))?,
                WithLocation {
                    value: RinToken::RRoundBracket,
                    ..
                }
            ) {
                self.base.consume();
                break;
            }
            self.base.expect_token(RinToken::Comma)?;
        }
        return Ok(params);
    }

    fn parse_term(&mut self) -> Result<WithLocation<Term>, CompilerError<RinToken>> {
        return match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(None))?
            .clone()
        {
            WithLocation {
                value: RinToken::Interger(interger),
                location,
            } => {
                self.base.consume();
                let value = match self
                    .base
                    .peek(0)
                    .cloned()
                    .ok_or(CompilerError::UnexpectedToken(None))?
                {
                    WithLocation {
                        value: RinToken::Type(typ),
                        location,
                    } => match typ {
                        PrimitiveType::U64 => Ok(Term::Literal(WithLocation::new(
                            Literal::U64(interger as u64),
                            location,
                        ))),
                        PrimitiveType::U32 => Ok(Term::Literal(WithLocation::new(
                            Literal::U32(interger as u32),
                            location,
                        ))),
                        PrimitiveType::U16 => Ok(Term::Literal(WithLocation::new(
                            Literal::U16(interger as u16),
                            location,
                        ))),
                        PrimitiveType::U8 => Ok(Term::Literal(WithLocation::new(
                            Literal::U8(interger as u8),
                            location,
                        ))),
                        _ => todo!(),
                    },
                    unexpected => Err(CompilerError::UnexpectedToken(Some(unexpected.clone()))),
                };
                self.base.consume();
                value.map(|e| WithLocation::new(e, location))
            }
            WithLocation {
                value: RinToken::String(string),
                location,
            } => {
                self.base.consume();
                Ok(WithLocation::new(
                    Term::Literal(WithLocation::new(Literal::String(string), location.clone())),
                    location,
                ))
            }
            WithLocation {
                value: RinToken::Keyword(keyword),
                location,
            } => match keyword {
                Keyword::True => {
                    self.base.consume();
                    Ok(WithLocation::new(
                        Term::Literal(WithLocation::new(Literal::Boolean(true), location.clone())),
                        location,
                    ))
                }
                Keyword::False => {
                    self.base.consume();
                    Ok(WithLocation::new(
                        Term::Literal(WithLocation::new(Literal::Boolean(false), location.clone())),
                        location,
                    ))
                }
                unexpected => Err(CompilerError::UnexpectedToken(Some(WithLocation::new(
                    RinToken::Keyword(unexpected),
                    location,
                )))),
            },
            WithLocation {
                value: RinToken::Identifier(ident),
                location,
            } => {
                if self.base.peek(1).is_some_and(|e| {
                    matches!(e.value(), RinToken::LRoundBracket)
                        || matches!(e.value(), RinToken::Dot)
                }) {
                    return Ok(self.parse_fn_call()?);
                }
                self.base.consume();

                Ok(WithLocation::new(
                    Term::LocalVariableAccess(WithLocation::new(ident, location.clone())),
                    location,
                ))
            }
            unexpected => Err(CompilerError::UnexpectedToken(Some(unexpected))),
        };
    }

    fn parse_fn_call(&mut self) -> Result<WithLocation<Term>, CompilerError<RinToken>> {
        let WithLocation {
            value: path,
            location,
        } = self.parse_path()?;
        self.base.expect_token(RinToken::LRoundBracket)?;
        let mut params = Vec::new();
        if self
            .base
            .peek(0)
            .is_some_and(|e| matches!(e.value(), RinToken::RRoundBracket))
        {
            self.base.consume();
            return Ok(WithLocation::new(
                Term::ProcedureCall(WithLocation::new(path, location.clone()), params),
                location,
            ));
        }

        while self.base.peek(0).is_some() {
            let param = self.parse_expression(0)?;
            params.push(param);
            if matches!(
                self.base
                    .peek(0)
                    .ok_or(CompilerError::UnexpectedToken(None,))?,
                WithLocation {
                    value: RinToken::RRoundBracket,
                    ..
                }
            ) {
                self.base.consume();
                break;
            }
            self.base.expect_token(RinToken::Comma)?;
        }

        return Ok(WithLocation::new(
            Term::ProcedureCall(WithLocation::new(path, location.clone()), params),
            location,
        ));
    }

    fn parse_imports(&mut self) -> Result<Import, CompilerError<RinToken>> {
        self.base.expect_token(RinToken::Keyword(Keyword::Import))?;
        let path = self.parse_path()?;
        self.base.expect_token(RinToken::Semicolon)?;
        return Ok(Import { path: path.value });
    }

    fn parse_module(&mut self) -> Result<Module, CompilerError<RinToken>> {
        self.base.expect_token(RinToken::Keyword(Keyword::Module))?;
        let name = self.parse_identifier()?;
        self.base.expect_token(RinToken::Semicolon)?;
        return Ok(Module { name: name.value });
    }

    fn parse_path(&mut self) -> Result<WithLocation<Path>, CompilerError<RinToken>> {
        let mut path = Vec::new();
        let mut location = None;
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                WithLocation {
                    value: RinToken::Identifier(ident),
                    location: loc,
                } => {
                    location = location.or_else(|| Some(loc));
                    self.base.consume();
                    path.push(ident);
                }
                WithLocation {
                    value: RinToken::Dot,
                    location: loc,
                } => {
                    location = location.or_else(|| Some(loc));
                    self.base.consume();
                    continue;
                }
                unexpected => return Err(CompilerError::UnexpectedToken(Some(unexpected))),
            }

            if !matches!(
                self.base
                    .peek(0)
                    .ok_or(CompilerError::UnexpectedToken(None,))?
                    .value(),
                RinToken::Dot
            ) {
                break;
            }
        }

        return Ok(WithLocation::new(
            Path { path },
            location.expect("This should not fail"),
        ));
    }

    fn parse_parameter(&mut self) -> Result<Parameter, CompilerError<RinToken>> {
        let param_type = self.parse_type()?;
        let name = self.parse_identifier()?;
        return Ok(Parameter { param_type, name });
    }

    fn parse_type(&mut self) -> Result<WithLocation<Type>, CompilerError<RinToken>> {
        match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(None))?
            .clone()
        {
            WithLocation {
                value: RinToken::Type(typ),
                location,
            } => {
                self.base.consume();
                Ok(WithLocation::new(typ.into(), location))
            }
            unexpected => Err(CompilerError::UnexpectedToken(Some(unexpected))),
        }
    }

    fn parse_identifier(&mut self) -> Result<WithLocation<String>, CompilerError<RinToken>> {
        match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(None))?
            .clone()
        {
            WithLocation {
                value: RinToken::Identifier(ident),
                location,
            } => {
                self.base.consume();
                Ok(WithLocation::new(ident, location))
            }
            unexpected => Err(CompilerError::UnexpectedToken(Some(unexpected))),
        }
    }
}

impl Path {
    pub fn new<T: AsRef<str>>(value: T) -> Self {
        Self {
            path: value.as_ref().split(".").map(|e| e.to_string()).collect(),
        }
    }

    pub fn join(self, other: Self) -> Self {
        Self {
            path: self
                .path
                .into_iter()
                .chain(other.path.into_iter())
                .collect::<Vec<String>>(),
        }
    }

    pub fn parse(&self) -> String {
        self.path.join("$")
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.join("."))
    }
}

impl Type {
    fn size(&self) -> TypeSizes {
        match self {
            Self::U64 | Self::I64 => TypeSizes::SizeU64,
            Self::U32 | Self::I32 => TypeSizes::SizeU32,
            Self::U16 | Self::I16 => TypeSizes::SizeU16,
            Self::U8 | Self::I8 | Self::Bool => TypeSizes::SizeU8,
            Self::Void => TypeSizes::SizeVoid,
        }
    }
}

impl TypeSizes {
    pub fn byte(&self) -> usize {
        return match self {
            TypeSizes::SizeU8 => 1,
            TypeSizes::SizeU16 => 2,
            TypeSizes::SizeU32 => 4,
            TypeSizes::SizeU64 => 8,
            TypeSizes::SizeVoid => 0,
        };
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U64 => write!(f, "u64"),
            Self::U32 => write!(f, "u32"),
            Self::U16 => write!(f, "u16"),
            Self::U8 => write!(f, "u8"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::Void => write!(f, "void"),
            Self::Bool => write!(f, "bool"),
        }
    }
}

impl From<PrimitiveType> for Type {
    fn from(primitive_type: PrimitiveType) -> Self {
        match primitive_type {
            PrimitiveType::Bool => Self::Bool,
            PrimitiveType::U64 => Self::U64,
            PrimitiveType::U32 => Self::U32,
            PrimitiveType::U16 => Self::U16,
            PrimitiveType::U8 => Self::U8,
            PrimitiveType::I8 => Self::I8,
            PrimitiveType::I16 => Self::I16,
            PrimitiveType::I32 => Self::I32,
            PrimitiveType::I64 => Self::I64,
        }
    }
}

impl From<Operator> for BinaryOperator {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Add => Self::Add,
            Operator::Multiply => Self::Multiply,
            Operator::Subtract => Self::Subtract,
            Operator::Divide => Self::Divide,
        }
    }
}
