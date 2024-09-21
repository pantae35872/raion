use crate::token::rin_token::{Keyword, Operator, PrimitiveType, RinToken};

use super::{CompilerBase, CompilerError};

#[derive(Default, Debug)]
pub struct Program {
    imports: Vec<Import>,
    module: Vec<Module>,
    procedures: Vec<Procedure>,
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
    name: String,
    return_type: Type,
    parameters: Vec<Parameter>,
    body: Vec<Statement>,
}

#[derive(Debug)]
struct Parameter {
    param_type: Type,
    name: String,
}

#[derive(Debug)]
enum Type {
    U64,
    U32,
    U16,
    U8,
    I8,
    I16,
    I32,
    I64,
    Bool,
    //Struct(String)
}

#[derive(Debug)]
enum Statement {
    Expression(Expression),
    Return(Expression),
    VariableDecl {
        name: String,
        var_type: Type,
        value: Expression,
    },
}

#[derive(Debug)]
enum Expression {
    Term(Term),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
}

#[derive(Debug)]
enum Term {
    Literial(Literal),
    ProcedureCall(Path, Vec<Expression>),
    LocalVariableAccess(String),
}

#[derive(Debug)]
enum Literal {
    Interger(u64),
    String(String),
}

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
struct Path {
    path: Vec<String>,
}

pub struct RinCompiler {
    base: CompilerBase<RinToken>,
    program: Program,
}

impl RinCompiler {
    pub fn new(tokens: Vec<RinToken>) -> Self {
        Self {
            base: CompilerBase::new(tokens),
            program: Program::default(),
        }
    }

    pub fn program(&self) -> &Program {
        return &self.program;
    }

    pub fn parse(&mut self) -> Result<(), CompilerError<RinToken>> {
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                RinToken::Keyword(keyword) => match keyword {
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
                        self.program.procedures.push(proc);
                    }
                    unexpected => {
                        return Err(CompilerError::UnexpectedToken(
                            Some(RinToken::Keyword(unexpected)),
                            self.base.current_line(),
                        ));
                    }
                },
                RinToken::NewLine => {
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
        let statements = self.parse_block()?;
        return Ok(Procedure {
            name,
            return_type,
            parameters: params,
            body: statements,
        });
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, CompilerError<RinToken>> {
        self.base.expect_token(RinToken::LCurly)?;
        let mut statements = Vec::new();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                RinToken::Keyword(_) | RinToken::Identifier(_) | RinToken::Type(_) => {
                    let statement = self.parse_statement()?;
                    statements.push(statement);
                }
                RinToken::RCurly => {
                    break;
                }
                RinToken::NewLine => {
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
        self.base.expect_token(RinToken::RCurly)?;
        return Ok(statements);
    }

    fn parse_statement(&mut self) -> Result<Statement, CompilerError<RinToken>> {
        let value = match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(
                None,
                self.base.current_line(),
            ))?
            .clone()
        {
            RinToken::Type(_) => {
                let var_type = self.parse_type()?;
                let name = self.parse_identifier()?;
                self.base.expect_token(RinToken::Equals)?;
                let value = self.parse_expression(0)?;
                Ok(Statement::VariableDecl {
                    name,
                    var_type,
                    value,
                })
            }
            RinToken::Keyword(keyword) => match keyword {
                Keyword::Return => {
                    self.base.consume();
                    let expr = self.parse_expression(0)?;
                    Ok(Statement::Return(expr))
                }
                unexpected => Err(CompilerError::UnexpectedToken(
                    Some(RinToken::Keyword(unexpected)),
                    self.base.current_line(),
                )),
            },
            _ => Ok(Statement::Expression(self.parse_expression(0)?)),
        };

        self.base.expect_token(RinToken::Semicolon)?;
        return value;
    }

    fn parse_expression(&mut self, min_prec: usize) -> Result<Expression, CompilerError<RinToken>> {
        let mut lhs = Expression::Term(self.parse_term()?);
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                RinToken::Operator(operator) => {
                    if operator.prec() < min_prec {
                        break;
                    }
                    self.base.consume();
                    let rhs = self.parse_expression(operator.prec() + 1)?;

                    lhs = Expression::BinaryOp(lhs.into(), operator.into(), rhs.into());
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
                RinToken::Type(_) => {
                    let param = self.parse_parameter()?;
                    params.push(param);
                }
                RinToken::RRoundBracket => {
                    self.base.consume();
                    break;
                }
                unexpected => {
                    return Err(CompilerError::UnexpectedToken(
                        Some(unexpected),
                        self.base.current_line(),
                    ))
                }
            }
            if matches!(
                self.base.peek(0).ok_or(CompilerError::UnexpectedToken(
                    None,
                    self.base.current_line()
                ))?,
                RinToken::RRoundBracket
            ) {
                self.base.consume();
                break;
            }
            self.base.expect_token(RinToken::Comma)?;
        }
        return Ok(params);
    }

    fn parse_term(&mut self) -> Result<Term, CompilerError<RinToken>> {
        return match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(
                None,
                self.base.current_line(),
            ))?
            .clone()
        {
            RinToken::Interger(interger) => {
                self.base.consume();
                Ok(Term::Literial(Literal::Interger(interger)))
            }
            RinToken::String(string) => {
                self.base.consume();
                Ok(Term::Literial(Literal::String(string)))
            }
            RinToken::Identifier(ident) => {
                if self.base.peek(1).is_some_and(|e| {
                    matches!(e, RinToken::LRoundBracket) || matches!(e, RinToken::Dot)
                }) {
                    return Ok(self.parse_fn_call()?);
                }
                self.base.consume();

                Ok(Term::LocalVariableAccess(ident))
            }
            unexpected => Err(CompilerError::UnexpectedToken(
                Some(unexpected),
                self.base.current_line(),
            )),
        };
    }

    fn parse_fn_call(&mut self) -> Result<Term, CompilerError<RinToken>> {
        let path = self.parse_path()?;
        self.base.expect_token(RinToken::LRoundBracket)?;
        let mut params = Vec::new();
        if self
            .base
            .peek(0)
            .is_some_and(|e| matches!(e, RinToken::RRoundBracket))
        {
            self.base.consume();
            return Ok(Term::ProcedureCall(path, params));
        }

        while self.base.peek(0).is_some() {
            let param = self.parse_expression(0)?;
            params.push(param);
            if matches!(
                self.base.peek(0).ok_or(CompilerError::UnexpectedToken(
                    None,
                    self.base.current_line()
                ))?,
                RinToken::RRoundBracket
            ) {
                self.base.consume();
                break;
            }
            self.base.expect_token(RinToken::Comma)?;
        }

        return Ok(Term::ProcedureCall(path, params));
    }

    fn parse_imports(&mut self) -> Result<Import, CompilerError<RinToken>> {
        self.base.expect_token(RinToken::Keyword(Keyword::Import))?;
        let path = self.parse_path()?;
        self.base.expect_token(RinToken::Semicolon)?;
        return Ok(Import { path });
    }

    fn parse_module(&mut self) -> Result<Module, CompilerError<RinToken>> {
        self.base.expect_token(RinToken::Keyword(Keyword::Module))?;
        let name = self.parse_identifier()?;
        self.base.expect_token(RinToken::Semicolon)?;
        return Ok(Module { name });
    }

    fn parse_path(&mut self) -> Result<Path, CompilerError<RinToken>> {
        let mut path = Vec::new();
        while let Some(token) = self.base.peek(0).cloned() {
            match token {
                RinToken::Identifier(ident) => {
                    self.base.consume();
                    path.push(ident);
                }
                RinToken::Dot => {
                    self.base.consume();
                    continue;
                }
                unexpected => {
                    return Err(CompilerError::UnexpectedToken(
                        Some(unexpected),
                        self.base.current_line(),
                    ))
                }
            }

            if !matches!(
                self.base.peek(0).ok_or(CompilerError::UnexpectedToken(
                    None,
                    self.base.current_line()
                ))?,
                RinToken::Dot
            ) {
                break;
            }
        }

        return Ok(Path { path });
    }

    fn parse_parameter(&mut self) -> Result<Parameter, CompilerError<RinToken>> {
        let param_type = self.parse_type()?;
        let name = self.parse_identifier()?;
        return Ok(Parameter { param_type, name });
    }

    fn parse_type(&mut self) -> Result<Type, CompilerError<RinToken>> {
        match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(
                None,
                self.base.current_line(),
            ))?
            .clone()
        {
            RinToken::Type(typ) => {
                self.base.consume();
                Ok(typ.into())
            }
            unexpected => Err(CompilerError::UnexpectedToken(
                Some(unexpected),
                self.base.current_line(),
            )),
        }
    }

    fn parse_identifier(&mut self) -> Result<String, CompilerError<RinToken>> {
        match self
            .base
            .peek(0)
            .ok_or(CompilerError::UnexpectedToken(
                None,
                self.base.current_line(),
            ))?
            .clone()
        {
            RinToken::Identifier(ident) => {
                self.base.consume();
                Ok(ident)
            }
            unexpected => Err(CompilerError::UnexpectedToken(
                Some(unexpected),
                self.base.current_line(),
            )),
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
