use std::{collections::HashMap, error::Error, fmt::Display};

use common::register::RegisterType;

use crate::token::rin_token::Operator;

use super::{BinaryOperator, Block, Expression, Literal, Procedure, RinAst, Statement, Term, Type};

#[derive(Debug)]
pub enum GeneratorError<'a> {
    UndefinedVariable(&'a String),
}

impl<'a> Display for GeneratorError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(var) => write!(f, "Use of undefined variable `{var}`"),
        }
    }
}

impl<'a> Error for GeneratorError<'a> {}

pub struct Generator {
    output: String,
}

struct ProcGenerator {
    stack_loc: usize,
    variables: HashMap<String, Variable>,
    body: String,
    constants: String,
    const_count: usize,
}

struct Variable {
    stack_loc: usize,
    var_type: Type,
}

enum ExpressionDestination {
    Register(RegisterType),
    Stack,
    Ignored,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    pub fn generate<'a>(mut self, ast: &'a RinAst) -> Result<String, GeneratorError<'a>> {
        self.output.push_str(&format!(
            "proc start -> {{\n   call main$main\n   exit a64\n}}\n"
        ));
        for procedure in ast.procedures.iter() {
            let proc_gen = ProcGenerator::new();
            self.output.push_str(&proc_gen.gen_proc(procedure)?);
        }
        return Ok(self.output);
    }
}

impl ProcGenerator {
    fn new() -> Self {
        Self {
            stack_loc: 0,
            variables: HashMap::new(),
            body: String::new(),
            constants: String::new(),
            const_count: 0,
        }
    }

    fn gen_proc<'a>(mut self, proc: &'a Procedure) -> Result<String, GeneratorError<'a>> {
        for (i, parameter) in proc.parameters.iter().enumerate() {
            self.variables.insert(
                parameter.name.clone(),
                Variable {
                    stack_loc: self.stack_loc,
                    var_type: parameter.param_type.clone(),
                },
            );
            self.add_instruction(format!("larg a64, {i}"));
            self.add_instruction(format!("mov [sp + {}], a64", self.stack_loc));
            self.stack_loc += 8;
        }
        self.gen_block(&proc.body)?;
        self.body
            .insert_str(0, &format!("   enter {}\n", self.stack_loc));
        self.body
            .insert_str(0, &format!("proc {} -> {{\n", proc.name.to_string()));
        self.body.push_str("}\n");
        self.body.push_str(&self.constants);
        return Ok(self.body);
    }

    fn gen_block<'a>(&mut self, block: &'a Block) -> Result<(), GeneratorError<'a>> {
        for statement in block.body.iter() {
            self.gen_statement(statement)?;
        }
        return Ok(());
    }

    fn gen_statement<'a>(&mut self, statement: &'a Statement) -> Result<(), GeneratorError<'a>> {
        match statement {
            Statement::VariableDecl {
                name,
                var_type,
                value,
            } => {
                self.variables.insert(
                    name.clone(),
                    Variable {
                        stack_loc: self.stack_loc,
                        var_type: var_type.clone(),
                    },
                );
                self.gen_expression(value, ExpressionDestination::Stack)?;
            }
            Statement::VariableMutate { name, value } => {
                self.gen_expression(value, ExpressionDestination::Register(RegisterType::A64))?;
                let variable = self
                    .variables
                    .get(name)
                    .ok_or(GeneratorError::UndefinedVariable(name))?;
                let stack_loc = variable.stack_loc;
                self.add_instruction(format!("mov [sp + {stack_loc}], a64"));
            }
            Statement::Return(expr) => {
                self.gen_expression(expr, ExpressionDestination::Register(RegisterType::A64))?;
                self.add_instruction("leave");
                self.add_instruction("ret");
            }
            Statement::Expression(expression) => {
                self.gen_expression(expression, ExpressionDestination::Ignored)?;
            }
        };
        return Ok(());
    }

    fn gen_expression<'a>(
        &mut self,
        expr: &'a Expression,
        dst: ExpressionDestination,
    ) -> Result<(), GeneratorError<'a>> {
        match expr {
            Expression::Term(term) => match term {
                Term::ProcedureCall(path, args) => {
                    for (i, arg) in args.iter().enumerate() {
                        self.gen_expression(
                            arg,
                            ExpressionDestination::Register(RegisterType::A64),
                        )?;
                        self.add_instruction(format!("arg {i}, a64"));
                    }
                    self.add_instruction(format!("call {}", path.to_string()));
                    match dst {
                        ExpressionDestination::Register(reg) => {
                            self.add_instruction(format!("mov {reg}, a64"));
                        }
                        ExpressionDestination::Stack => {
                            self.add_instruction(format!("mov [sp + {}], a64", self.stack_loc));
                            self.stack_loc += 8;
                        }
                        ExpressionDestination::Ignored => {}
                    }
                }
                Term::Literial(value) => match value {
                    Literal::String(value) => match dst {
                        ExpressionDestination::Register(reg) => {
                            let name = self.add_str_const(value);
                            self.add_instruction(format!("mov {reg}, {name}"))
                        }
                        ExpressionDestination::Stack => {
                            let name = self.add_str_const(value);
                            self.add_instruction(format!("mov a64, {name}"));
                            self.add_instruction(format!("mov [sp + {}], a64", self.stack_loc));
                            self.stack_loc += 8;
                        }
                        ExpressionDestination::Ignored => {}
                    },
                    Literal::Interger(int) => match dst {
                        ExpressionDestination::Register(reg) => {
                            self.add_instruction(format!("mov {reg}, {int}"))
                        }
                        ExpressionDestination::Stack => {
                            self.add_instruction(format!("mov a64, {int}"));
                            self.add_instruction(format!("mov [sp + {}], a64", self.stack_loc));
                            self.stack_loc += 8;
                        }
                        ExpressionDestination::Ignored => {}
                    },
                },
                Term::LocalVariableAccess(name) => {
                    let variable = self
                        .variables
                        .get(name)
                        .ok_or(GeneratorError::UndefinedVariable(name))?;
                    let stack_loc = variable.stack_loc;
                    match dst {
                        ExpressionDestination::Register(reg) => {
                            self.add_instruction(format!("mov {reg}, [sp + {stack_loc}]"));
                        }
                        ExpressionDestination::Stack => {}
                        ExpressionDestination::Ignored => {}
                    }
                }
            },
            Expression::BinaryOp(lhs, operator, rhs) => {
                self.gen_expression(&lhs, ExpressionDestination::Register(RegisterType::A64))?;
                self.gen_expression(&rhs, ExpressionDestination::Register(RegisterType::B64))?;
                match operator {
                    BinaryOperator::Add => self.add_instruction("add a64, b64"),
                    BinaryOperator::Subtract => self.add_instruction("sub a64, b64"),
                    BinaryOperator::Divide => todo!(),
                    BinaryOperator::Multiply => todo!(),
                }
                match dst {
                    ExpressionDestination::Register(reg) => {
                        if reg != RegisterType::A64 {
                            self.add_instruction(format!("mov {reg}, a64"));
                        }
                    }
                    ExpressionDestination::Stack => {
                        self.add_instruction(format!("mov [sp + {}], a64", self.stack_loc));
                        self.stack_loc += 8;
                    }
                    ExpressionDestination::Ignored => {}
                }
            }
        }
        return Ok(());
    }

    fn add_instruction<T: AsRef<str> + Display>(&mut self, value: T) {
        self.body.push_str(&format!("   {value}\n"));
    }

    fn add_str_const<T: AsRef<str> + Display>(&mut self, value: T) -> String {
        self.constants.push_str(&format!(
            "const const_{} -> {{\n\"{value}\\0\"\n}}\n",
            self.const_count
        ));
        let result = format!("const_{}", self.const_count);
        self.const_count += 1;
        return result;
    }
}
