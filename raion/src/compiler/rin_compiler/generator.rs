use std::{collections::HashMap, error::Error, fmt::Display};

use common::{
    inline_if,
    register::{RegisterType, RegisterTypeGroup},
};

use super::{
    BinaryOperator, Block, Expression, Literal, Path, Procedure, RinAst, Statement, Term, Type,
};

mod block_generator;

#[derive(Debug)]
pub enum GeneratorError<'a> {
    UndefinedVariable(&'a String),
    UndefinedProcedure(&'a Path),
    UnexpectedType { expected: Type, unexpected: Type },
}

impl<'a> Display for GeneratorError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(var) => write!(f, "Use of undefined variable `{var}`"),
            Self::UndefinedProcedure(proc) => write!(f, "Use of undefined procedure `{proc}`"),
            Self::UnexpectedType {
                expected,
                unexpected,
            } => write!(
                f,
                "Unexpected type expected: `{expected}` found: `{unexpected}`"
            ),
        }
    }
}

impl<'a> Error for GeneratorError<'a> {}

type Variables = HashMap<String, Variable>;

pub struct Generator {
    output: String,
    callable_procs: Vec<ProcHeader>,
}

struct ProcGenerator<'a> {
    stack_loc: usize,
    variables: Variables,
    body: String,
    constants: String,
    const_count: usize,
    callable_procs: &'a Vec<ProcHeader>,
}

struct ProcHeader {
    name: Path,
    parameters: Vec<Type>,
    return_type: Type,
}

#[derive(Clone)]
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
            callable_procs: Vec::new(),
        }
    }

    pub fn generate<'a>(mut self, ast: &'a RinAst) -> Result<String, GeneratorError<'a>> {
        self.output.push_str(&format!(
            "proc start -> {{\n   call main$main\n   exit a64\n}}\n"
        ));
        for procedure in ast.procedures.iter() {
            let param_type = procedure.parameters.iter().map(|e| e.param_type).collect();
            self.callable_procs.push(ProcHeader {
                name: procedure.name.clone(),
                parameters: param_type,
                return_type: procedure.return_type,
            });
        }
        for procedure in ast.procedures.iter() {
            let proc_gen = ProcGenerator::new(&self.callable_procs);
            self.output.push_str(&proc_gen.gen_proc(procedure)?);
        }
        return Ok(self.output);
    }
}

impl<'a> ProcGenerator<'a> {
    fn new(callable_procs: &'a Vec<ProcHeader>) -> Self {
        Self {
            stack_loc: 0,
            variables: HashMap::new(),
            body: String::new(),
            constants: String::new(),
            callable_procs,
            const_count: 0,
        }
    }

    fn gen_proc<'b>(mut self, proc: &'b Procedure) -> Result<String, GeneratorError<'b>> {
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
        let return_type = self.gen_block(&proc.body)?;
        self.body
            .insert_str(0, &format!("   enter {}\n", self.stack_loc));
        self.body
            .insert_str(0, &format!("proc {} -> {{\n", proc.name.to_string()));
        self.body.push_str("}\n");
        self.body.push_str(&self.constants);
        if proc.return_type != return_type {
            return Err(GeneratorError::UnexpectedType {
                expected: proc.return_type.clone(),
                unexpected: return_type,
            });
        }
        return Ok(self.body);
    }

    fn gen_block<'b>(&mut self, block: &'b Block) -> Result<Type, GeneratorError<'b>> {
        let mut return_type = Type::Void;
        for statement in block.body.iter() {
            if let Some(stmt_type) = self.gen_statement(statement)? {
                if return_type != Type::Void && return_type != stmt_type {
                    return Err(GeneratorError::UnexpectedType {
                        expected: return_type,
                        unexpected: stmt_type,
                    });
                }
                return_type = stmt_type;
            }
        }
        return Ok(return_type);
    }

    fn gen_statement<'b>(
        &mut self,
        statement: &'b Statement,
    ) -> Result<Option<Type>, GeneratorError<'b>> {
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
                let expr_type = self.gen_expression(value, ExpressionDestination::Stack, &[])?;
                if expr_type != *var_type {
                    return Err(GeneratorError::UnexpectedType {
                        expected: var_type.clone(),
                        unexpected: expr_type,
                    });
                }
            }
            Statement::VariableMutate { name, value } => {
                self.gen_expression(
                    value,
                    ExpressionDestination::Register(RegisterType::A64),
                    &[],
                )?;
                let variable = self
                    .variables
                    .get(name)
                    .ok_or(GeneratorError::UndefinedVariable(name))?;
                let stack_loc = variable.stack_loc;
                self.add_instruction(format!("mov [sp + {stack_loc}], a64"));
            }
            Statement::Return(expr) => {
                let typ = self.gen_expression(
                    expr,
                    ExpressionDestination::Register(RegisterType::A64),
                    &[],
                )?;
                self.add_instruction("leave");
                self.add_instruction("ret");
                return Ok(Some(typ));
            }
            Statement::Expression(expression) => {
                self.gen_expression(expression, ExpressionDestination::Ignored, &[])?;
            }
        };
        return Ok(None);
    }

    fn gen_expression<'b>(
        &mut self,
        expr: &'b Expression,
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<Type, GeneratorError<'b>> {
        match expr {
            Expression::Term(term) => self.gen_term(term, dst, preserved_registers),
            Expression::BinaryOp(lhs, operator, rhs) => {
                self.gen_binary_op(lhs, operator, rhs, dst, preserved_registers)
            }
        }
    }

    fn gen_binary_op<'b>(
        &mut self,
        lhs: &'b Expression,
        operator: &BinaryOperator,
        rhs: &'b Expression,
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<Type, GeneratorError<'b>> {
        self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
        let lhs_type =
            self.gen_expression(lhs, ExpressionDestination::Register(RegisterType::A64), &[])?;
        let rhs_type = self.gen_expression(
            rhs,
            ExpressionDestination::Register(RegisterType::B64),
            &[RegisterTypeGroup::A],
        )?;
        if lhs_type != rhs_type {
            return Err(GeneratorError::UnexpectedType {
                expected: lhs_type,
                unexpected: rhs_type,
            });
        }
        self.add_binary_op_instruction(operator);
        let expr_type = self.finalize_expression_result(dst, lhs_type);
        self.restore_registers(preserved_registers, &[RegisterTypeGroup::A]);
        expr_type
    }

    fn add_binary_op_instruction(&mut self, operator: &BinaryOperator) {
        match operator {
            BinaryOperator::Add => self.add_instruction("add a64, b64"),
            BinaryOperator::Subtract => self.add_instruction("sub a64, b64"),
            BinaryOperator::Multiply => self.add_instruction("mul a64, b64"),
            BinaryOperator::Divide => self.add_instruction("div a64, b64"),
        }
    }

    fn gen_term<'b>(
        &mut self,
        term: &'b Term,
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<Type, GeneratorError<'b>> {
        match term {
            Term::ProcedureCall(path, args) => {
                self.gen_procedure_call(path, args, dst, preserved_registers)
            }
            Term::Literal(value) => self.gen_literal(value, dst, preserved_registers),
            Term::LocalVariableAccess(name) => self.gen_variable_access(name, dst),
        }
    }

    fn gen_variable_access<'b>(
        &mut self,
        name: &'b String,
        dst: ExpressionDestination,
    ) -> Result<Type, GeneratorError<'b>> {
        let variable = self
            .variables
            .get(name)
            .ok_or(GeneratorError::UndefinedVariable(name))
            .cloned()?;
        let var_type = variable.var_type.clone();
        self.gen_variable_load_instruction(variable, dst, var_type.clone())?;
        Ok(var_type)
    }

    fn gen_variable_load_instruction<'b>(
        &mut self,
        variable: Variable,
        dst: ExpressionDestination,
        var_type: Type,
    ) -> Result<(), GeneratorError<'b>> {
        match dst {
            ExpressionDestination::Register(reg) => match reg.group() {
                RegisterTypeGroup::A => self.add_instruction(format!(
                    "mov a{}, [sp + {}]",
                    var_type.size().byte() * 8,
                    variable.stack_loc
                )),
                RegisterTypeGroup::B => self.add_instruction(format!(
                    "mov b{}, [sp + {}]",
                    var_type.size().byte() * 8,
                    variable.stack_loc
                )),
                RegisterTypeGroup::C => self.add_instruction(format!(
                    "mov c{}, [sp + {}]",
                    var_type.size().byte() * 8,
                    variable.stack_loc
                )),
                RegisterTypeGroup::D => self.add_instruction(format!(
                    "mov d{}, [sp + {}]",
                    var_type.size().byte() * 8,
                    variable.stack_loc
                )),
                _ => todo!(),
            },
            ExpressionDestination::Stack => {
                let stack_loc = variable.stack_loc;
                let size = var_type.size().byte();
                let bits = size * 8;
                self.add_instruction(format!("mov a{bits}, [sp + {stack_loc}]"));
                self.add_instruction(format!("mov [sp + {}], a{bits}", self.stack_loc));
                self.stack_loc += size;
            }
            ExpressionDestination::Ignored => {}
        }
        Ok(())
    }

    fn gen_literal<'b>(
        &mut self,
        literal: &'b Literal,
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<Type, GeneratorError<'b>> {
        let const_label = format!("c{}", self.const_count);
        let typ = match literal {
            Literal::String(value) => {
                self.constants
                    .push_str(&format!("{const_label}: .str \"{value}\"\n"));
                self.const_count += 1;
                self.add_instruction(format!("mov a64, {const_label}"));
                todo!()
            }
            Literal::U64(value) => {
                self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
                self.add_instruction(format!("mov a64, {value}"));
                Type::U64
            }
            Literal::U32(value) => {
                self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
                self.add_instruction(format!("mov a32, {value}"));
                Type::U32
            }
            Literal::U16(value) => {
                self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
                self.add_instruction(format!("mov a16, {value}"));
                Type::U16
            }
            Literal::U8(value) => {
                self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
                self.add_instruction(format!("mov a8, {value}"));
                Type::U8
            }
            Literal::Boolean(value) => {
                self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
                self.add_instruction(format!("mov a8, {}", inline_if!(*value, 1, 0)));
                Type::Bool
            }
        };
        let expr_type = self.finalize_expression_result(dst, typ);
        self.restore_registers(preserved_registers, &[RegisterTypeGroup::A]);
        expr_type
    }

    fn preserve_registers(
        &mut self,
        registers: &[RegisterTypeGroup],
        used_register: &[RegisterTypeGroup],
    ) {
        for reg in registers
            .iter()
            .filter(|e| used_register.iter().find(|e1| e == e1).is_some())
        {
            self.add_instruction(format!("savr {reg}64"));
        }
    }

    fn restore_registers(
        &mut self,
        registers: &[RegisterTypeGroup],
        used_register: &[RegisterTypeGroup],
    ) {
        for reg in registers
            .iter()
            .filter(|e| used_register.iter().find(|e1| e == e1).is_some())
            .rev()
        {
            self.add_instruction(format!("restr {reg}64"));
        }
    }

    fn gen_procedure_call<'b>(
        &mut self,
        path: &'b Path,
        args: &'b [Expression],
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<Type, GeneratorError<'b>> {
        let proc = self
            .callable_procs
            .iter()
            .find(|e| e.name == *path)
            .ok_or(GeneratorError::UndefinedProcedure(path))?;
        for (i, (expr, param_type)) in args.iter().zip(proc.parameters.iter()).enumerate() {
            let expr_type = self.gen_expression(
                expr,
                ExpressionDestination::Register(RegisterType::A64),
                &[],
            )?;
            if expr_type != *param_type {
                return Err(GeneratorError::UnexpectedType {
                    expected: *param_type,
                    unexpected: expr_type,
                });
            }
            self.add_instruction(format!("arg {i}, a64"));
        }
        self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
        self.add_instruction(format!("call {path}"));
        let proc_return_type = self.finalize_expression_result(dst, proc.return_type)?;
        self.restore_registers(preserved_registers, &[RegisterTypeGroup::A]);
        return Ok(proc_return_type);
    }

    fn finalize_expression_result<'b>(
        &mut self,
        dst: ExpressionDestination,
        expr_type: Type,
    ) -> Result<Type, GeneratorError<'b>> {
        match dst {
            ExpressionDestination::Register(reg) if reg.group() != RegisterTypeGroup::A => {
                self.add_instruction(format!("mov {reg}, a64"));
            }
            ExpressionDestination::Stack => {
                let expr_size = expr_type.size().byte();
                let expr_bits = expr_size * 8;
                if expr_size != 0 {
                    self.add_instruction(format!("mov [sp + {}], a{expr_bits}", self.stack_loc));
                }
                self.stack_loc += expr_type.size().byte();
            }
            ExpressionDestination::Register(_) => {}
            ExpressionDestination::Ignored => {}
        }
        Ok(expr_type)
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
