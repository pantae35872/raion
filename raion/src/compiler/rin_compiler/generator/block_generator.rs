use std::fmt::Display;

use common::{
    inline_if,
    register::{RegisterType, RegisterTypeGroup},
};

use crate::{
    compiler::rin_compiler::{
        BinaryOperator, Block, Expression, Literal, Path, Statement, Term, Type,
    },
    WithLocation,
};

use super::{ExpressionDestination, GeneratorError, ProcHeader, Variable, Variables};

pub struct BlockGenerator<'a> {
    stack_loc: &'a mut usize,
    variables: &'a Variables,
    local_variables: Variables,
    body: String,
    callable_procs: &'a Vec<ProcHeader>,
}

pub enum ReturnDestion {
    Register(RegisterType),
    LeaveReturn,
    Return,
}

impl<'a> BlockGenerator<'a> {
    pub fn new(
        variables: &'a Variables,
        stack_loc: &'a mut usize,
        callable_procs: &'a Vec<ProcHeader>,
    ) -> Self {
        Self {
            variables,
            stack_loc,
            local_variables: Variables::new(),
            body: String::new(),
            callable_procs,
        }
    }

    pub fn gen_block<'b>(
        mut self,
        block: &'b WithLocation<Block>,
        return_dst: ReturnDestion,
    ) -> Result<(WithLocation<Type>, String), GeneratorError<'b>> {
        let WithLocation {
            value: block,
            location,
        } = block;
        let mut return_type = WithLocation::new(Type::Void, location.clone());
        for statement in block.body.iter() {
            if let Some(stmt_type) = self.gen_statement(statement, &return_dst)? {
                if return_type.value != Type::Void && return_type.value != *stmt_type {
                    return Err(GeneratorError::UnexpectedType {
                        expected: return_type,
                        unexpected: stmt_type,
                    });
                }
                return_type = stmt_type;
            }
        }
        return Ok((return_type, self.body));
    }

    fn gen_statement<'b>(
        &mut self,
        statement: &'b Statement,
        return_type: &ReturnDestion,
    ) -> Result<Option<WithLocation<Type>>, GeneratorError<'b>> {
        match statement {
            Statement::VariableDecl { name, value } => {
                let stack_loc = *self.stack_loc;
                let expr_type = self.gen_expression(value, ExpressionDestination::Stack, &[])?;
                self.local_variables.insert(
                    name.value.clone(),
                    Variable {
                        stack_loc,
                        var_type: expr_type,
                    },
                );
            }
            Statement::VariableMutate { name, value } => {
                let expr_type = self.gen_expression(
                    value,
                    ExpressionDestination::Register(RegisterType::A64),
                    &[],
                )?;
                let variable = self.get_variable(name)?;
                if expr_type.value != variable.var_type.value {
                    return Err(GeneratorError::UnexpectedType {
                        expected: variable.var_type,
                        unexpected: expr_type,
                    });
                }
                let stack_loc = variable.stack_loc;
                let bits = variable.var_type.size().byte() * 8;
                self.add_instruction(format!("mov [sp + {stack_loc}], a{bits}"));
            }
            Statement::Return(expr) => {
                let return_type = match return_type {
                    ReturnDestion::Register(reg) => {
                        let typ =
                            self.gen_expression(expr, ExpressionDestination::Register(*reg), &[])?;
                        typ
                    }
                    ReturnDestion::Return => {
                        let typ = self.gen_expression(
                            expr,
                            ExpressionDestination::Register(RegisterType::A64),
                            &[],
                        )?;
                        self.add_instruction("ret");
                        typ
                    }
                    ReturnDestion::LeaveReturn => {
                        let typ = self.gen_expression(
                            expr,
                            ExpressionDestination::Register(RegisterType::A64),
                            &[],
                        )?;
                        self.add_instruction("leave");
                        self.add_instruction("ret");
                        typ
                    }
                };
                return Ok(Some(return_type));
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
    ) -> Result<WithLocation<Type>, GeneratorError<'b>> {
        match expr {
            Expression::Term(term) => self.gen_term(term, dst, preserved_registers),
            Expression::BinaryOp(lhs, operator, rhs) => {
                self.gen_binary_op(lhs, operator, rhs, dst, preserved_registers)
            }
        }
    }

    fn gen_binary_op<'b>(
        &mut self,
        lhs: &'b WithLocation<Expression>,
        operator: &BinaryOperator,
        rhs: &'b Expression,
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<WithLocation<Type>, GeneratorError<'b>> {
        let WithLocation {
            value: lhs,
            location,
        } = lhs;
        self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
        let lhs_type =
            self.gen_expression(lhs, ExpressionDestination::Register(RegisterType::A64), &[])?;
        let rhs_type = self.gen_expression(
            rhs,
            ExpressionDestination::Register(RegisterType::B64),
            &[RegisterTypeGroup::A],
        )?;
        if lhs_type.value != rhs_type.value {
            return Err(GeneratorError::UnexpectedType {
                expected: WithLocation::new(lhs_type.value, location.clone()),
                unexpected: rhs_type,
            });
        }
        self.add_binary_op_instruction(operator);
        let expr_type = self.finalize_expression_result(dst, lhs_type.value);
        self.restore_registers(preserved_registers, &[RegisterTypeGroup::A]);
        expr_type.map(|e| WithLocation::new(e, location.clone()))
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
    ) -> Result<WithLocation<Type>, GeneratorError<'b>> {
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
        name: &'b WithLocation<String>,
        dst: ExpressionDestination,
    ) -> Result<WithLocation<Type>, GeneratorError<'b>> {
        let variable = self.get_variable(name)?;
        let var_type = variable.var_type.clone();
        self.gen_variable_load_instruction(variable, dst, var_type.value.clone())?;
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
                *self.stack_loc += size;
            }
            ExpressionDestination::Ignored => {}
        }
        Ok(())
    }

    fn gen_literal<'b>(
        &mut self,
        literal: &'b WithLocation<Literal>,
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<WithLocation<Type>, GeneratorError<'b>> {
        let WithLocation {
            value: literal,
            location,
        } = literal;
        let typ = match literal {
            Literal::String(_value) => todo!(),
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
        expr_type.map(|e| WithLocation::new(e, location.clone()))
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
        path: &'b WithLocation<Path>,
        args: &'b [WithLocation<Expression>],
        dst: ExpressionDestination,
        preserved_registers: &[RegisterTypeGroup],
    ) -> Result<WithLocation<Type>, GeneratorError<'b>> {
        let proc = self
            .callable_procs
            .iter()
            .find(|e| e.callable_path == path.value)
            .ok_or(GeneratorError::UndefinedProcedure(path))?;
        let WithLocation { location, .. } = path;
        // TODO: Check for parameter length
        self.preserve_registers(preserved_registers, &[RegisterTypeGroup::A]);
        for (i, (expr, param_type)) in args.iter().zip(proc.parameters.iter()).enumerate() {
            let expr_type = self.gen_expression(
                expr,
                ExpressionDestination::Register(RegisterType::A64),
                &[],
            )?;
            if expr_type.value != param_type.value {
                return Err(GeneratorError::UnexpectedType {
                    expected: param_type.clone(),
                    unexpected: expr_type,
                });
            }
            self.add_instruction(format!("arg {i}, a64"));
        }
        self.add_instruction(format!("call {}", proc.real_path.parse()));
        let proc_return_type = self.finalize_expression_result(dst, proc.return_type)?;
        self.restore_registers(preserved_registers, &[RegisterTypeGroup::A]);
        return Ok(WithLocation::new(proc_return_type, location.clone()));
    }

    fn get_variable<'b>(
        &self,
        name: &'b WithLocation<String>,
    ) -> Result<Variable, GeneratorError<'b>> {
        let variable = self
            .variables
            .get(&name.value)
            .or_else(|| self.local_variables.get(&name.value))
            .ok_or(GeneratorError::UndefinedVariable(name))
            .cloned()?;
        return Ok(variable);
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
                *self.stack_loc += expr_type.size().byte();
            }
            ExpressionDestination::Register(_) => {}
            ExpressionDestination::Ignored => {}
        }
        Ok(expr_type)
    }

    fn add_instruction<T: AsRef<str> + Display>(&mut self, value: T) {
        self.body.push_str(&format!("   {value}\n"));
    }
}
