use std::{collections::HashMap, error::Error, fmt::Display};

use block_generator::{BlockGenerator, ReturnDestion};
use common::{register::RegisterType, VecUtils};

use crate::{error::ErrorGenerator, WithLocation};

use super::{Parameter, Path, Procedure, RinAst, Type};
use inline_colorization::*;

mod block_generator;

#[derive(Debug)]
pub enum GeneratorError<'a> {
    UndefinedVariable(&'a WithLocation<String>),
    UndefinedProcedure(&'a WithLocation<Path>),
    UnexpectedType {
        expected: WithLocation<Type>,
        unexpected: WithLocation<Type>,
    },
}

impl<'a> Display for GeneratorError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(WithLocation {
                value: var,
                location,
            }) => write!(
                f,
                "{}",
                ErrorGenerator::new(
                    location,
                    format!("{style_bold}Use of undefined variable `{var}`{style_reset}"),
                    location.column.to_string().len()
                )
                .vertical_pipe(format!("{}", location.column))?
                .write_line(location.column)?
                .new_line()?
                .vertical_pipe("")?
                .pointer(location.row, "", '^', color_red)?
                .build()
            ),
            Self::UndefinedProcedure(WithLocation {
                value: proc,
                location,
            }) => write!(
                f,
                "{}",
                ErrorGenerator::new(
                    location,
                    format!("{style_bold}Use of undefined procedure `{proc}`{style_reset}"),
                    location.column.to_string().len()
                )
                .vertical_pipe(format!("{}", location.column))?
                .write_line(location.column)?
                .new_line()?
                .vertical_pipe("")?
                .pointer(location.row, "", '^', color_red)?
                .build()
            ),
            Self::UnexpectedType {
                expected:
                    WithLocation {
                        value: expected,
                        location,
                    },
                unexpected:
                    WithLocation {
                        value: unexpected,
                        location: unexpected_location,
                    },
            } => {
                if location.column() == unexpected_location.column() {
                    write!(f, "{}", ErrorGenerator::new(
                        unexpected_location,
                        format!("{style_bold}Unexpected type. expected `{expected}` found `{unexpected}`{style_reset}"),
                        unexpected_location
                            .column
                            .to_string()
                            .len()
                        )
                        .vertical_pipe(format!("{}", unexpected_location.column))?
                        .write_line(unexpected_location.column)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(location.row, "", '^', color_blue)?
                        .pointer(unexpected_location.row - location.row - 1, format!(" expected `{expected}`, found `{unexpected}`"), '^', color_red)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(location.row, "", '|', color_blue)?
                        .new_line()?
                        .vertical_pipe("")?
                        .ident_string(location.row, "expected due to this", color_blue)?.build())
                } else {
                    write!(f, "{}", ErrorGenerator::new(
                        unexpected_location,
                        format!("{style_bold}Unexpected type. expected `{expected}` found `{unexpected}`{style_reset}"),
                        unexpected_location
                            .column
                            .to_string()
                            .len()
                            .max(location.column.to_string().len()),
                        ).vertical_pipe(format!("{}", unexpected_location.column))?
                        .write_line(unexpected_location.column)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(unexpected_location.row, format!(" expected `{expected}`, found `{unexpected}`"), '^', color_red)?
                        .new_line()?
                        .vertical_pipe(format!("{}", location.column))?
                        .write_line(location.column)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(location.row, "", '^', color_blue)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(location.row, "", '|', color_blue)?
                        .new_line()?
                        .vertical_pipe("")?
                        .ident_string(location.row, "expected due to this", color_blue)?.build())
                }
            }
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
    callable_procs: &'a Vec<ProcHeader>,
}

struct ProcHeader {
    callable_path: Path,
    real_path: Path,
    parameters: Vec<WithLocation<Type>>,
    return_type: Type,
}

#[derive(Clone)]
struct Variable {
    stack_loc: usize,
    var_type: WithLocation<Type>,
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

    pub fn generate<'a>(
        mut self,
        ast: &'a RinAst,
        package_path: &Path,
    ) -> Result<String, GeneratorError<'a>> {
        self.output.push_str(&format!(
            "proc start -> {{\n   call {package_path}$main\n   exit a64\n}}\n"
        ));
        for procedure in ast.procedures.iter() {
            let param_type = procedure
                .parameters
                .iter()
                .map(|e| e.param_type.clone())
                .collect();
            self.callable_procs.push(ProcHeader {
                real_path: package_path.clone().join(Path::new(&procedure.name.value)),
                callable_path: Path::new(&procedure.name.value),
                parameters: param_type,
                return_type: *procedure.return_type,
            });
        }
        for (procedure, header) in ast.procedures.iter().zip(self.callable_procs.iter()) {
            let proc_gen = ProcGenerator::new(&self.callable_procs);
            self.output.push_str(&proc_gen.gen_proc(procedure, header)?);
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
        }
    }

    fn gen_proc<'b>(
        mut self,
        proc: &'b Procedure,
        header: &ProcHeader,
    ) -> Result<String, GeneratorError<'b>> {
        self.gen_argument(&proc.parameters);
        let (return_type, generated) =
            BlockGenerator::new(&self.variables, &mut self.stack_loc, &self.callable_procs)
                .gen_block(&proc.body, ReturnDestion::LeaveReturn)?;
        self.body.push_str(&generated);
        self.body.push_str("}\n");
        self.body.push_str(&self.constants);
        self.body
            .insert_str(0, &format!("   enter {}\n", self.stack_loc));
        self.body
            .insert_str(0, &format!("proc {} -> {{\n", header.real_path.parse()));
        if proc.return_type.value != return_type.value {
            return Err(GeneratorError::UnexpectedType {
                expected: proc.return_type.clone(),
                unexpected: return_type,
            });
        }
        return Ok(self.body);
    }

    fn gen_argument(&mut self, parameters: &Vec<Parameter>) {
        for (i, parameter) in parameters.iter().enumerate() {
            self.variables.insert(
                parameter.name.value.clone(),
                Variable {
                    stack_loc: self.stack_loc,
                    var_type: parameter.param_type.clone(),
                },
            );
            self.add_instruction(format!("larg a64, {i}"));
            let bits = parameter.param_type.size().byte() * 8;
            self.add_instruction(format!("mov [sp + {}], a{bits}", self.stack_loc));
            self.stack_loc += parameter.param_type.size().byte();
        }
    }

    fn add_instruction<T: AsRef<str> + Display>(&mut self, value: T) {
        self.body.push_str(&format!("   {value}\n"));
    }
}
