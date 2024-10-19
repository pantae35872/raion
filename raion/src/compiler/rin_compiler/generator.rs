use std::{collections::HashMap, error::Error, fmt::Display};

use block_generator::{BlockGenerator, ReturnDestion};
use common::register::{RegisterType, RegisterTypeGroup};

use crate::{error::ErrorGenerator, WithLocation};

use super::{Expression, Parameter, Path, Procedure, RinAst, Type};
use inline_colorization::*;

mod block_generator;

#[derive(Debug)]
pub enum GeneratorError<'a> {
    UndefinedVariable(&'a WithLocation<String>),
    UndefinedProcedure(&'a WithLocation<Path>),
    UndefinedModule(&'a WithLocation<Path>),
    TooMuchArguments(
        &'a WithLocation<Path>,
        &'a WithLocation<Expression>,
        usize,
        usize,
        usize,
    ),
    TooFewArguemnts(&'a WithLocation<Path>, usize, usize, usize),
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
            Self::UndefinedModule(WithLocation {
                value: module,
                location,
            }) => write!(
                f,
                "{}",
                ErrorGenerator::new(
                    location,
                    format!("{style_bold}Import of an undefined module `{module}`{style_reset}"),
                    location.column.to_string().len()
                )
                .vertical_pipe(format!("{}", location.column))?
                .write_line(location.column)?
                .new_line()?
                .vertical_pipe("")?
                .pointer(location.row, "", '^', color_red)?
                .build()
            ),
            Self::TooMuchArguments(
                WithLocation {
                    location: proc_location,
                    ..
                },
                WithLocation {
                    location: arg_location,
                    ..
                },
                args_len,
                param_len,
                unexpected,
            ) => {
                if proc_location.column() == arg_location.column()
                    && proc_location.file() == arg_location.file()
                {
                    write!(f, "{}", ErrorGenerator::new(
                        proc_location,
                        format!("{style_bold}Too much arguments. Expected {param_len} arguments but found {args_len} arguments{style_reset}"),
                        proc_location
                            .column
                            .to_string()
                            .len()
                        )
                        .vertical_pipe(format!("{}", proc_location.column))?
                        .write_line(proc_location.column)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(proc_location.row, "", '^', color_blue)?
                        .pointer(arg_location.row - proc_location.row - 1, format!(" Unexpected argument #{unexpected}"), '^', color_red)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(proc_location.row, "", '|', color_blue)?
                        .new_line()?
                        .vertical_pipe("")?
                        .ident_string(proc_location.row, format!("Takes {param_len} arguments."), color_blue)?
                    .build())
                } else {
                    write!(f, "{}", ErrorGenerator::new(
                        arg_location, 
                        format!("{style_bold}Too much arguments. Expected {param_len} arguments but found {args_len} arguments{style_reset}"),
                        proc_location.column.to_string().len().max(arg_location.column.to_string().len()))
                        .vertical_pipe(format!("{}", arg_location.column))?
                        .write_line(arg_location.column)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(arg_location.row, format!(" Unexpected argument #{unexpected}`"), '^', color_red)?
                        .new_line()?
                        .vertical_pipe(proc_location.column.to_string())?
                        .write_line(proc_location.column)?
                        .new_line()?
                        .vertical_pipe("")?
                        .pointer(proc_location.row, format!(" Takes {param_len} arguments."), '^', color_blue)?
                        .build()
                    )
                }
            }
            Self::TooFewArguemnts(WithLocation {
                value: _proc,
                location,
            }, args_len, param_len, missing) => 
                write!(f, "{}", ErrorGenerator::new(
                    location,
                    format!("{style_bold}Too few arguments. Expected {param_len} arguments but found {args_len} arguments{style_reset}"),
                    location
                        .column
                        .to_string()
                        .len()
                    )
                    .vertical_pipe(format!("{}", location.column))?
                    .write_line(location.column)?
                    .new_line()?
                    .vertical_pipe("")?
                    .pointer(location.row, format!(" argument #{missing} is missing"), '^', color_red)?.build()),
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
                if location.column() == unexpected_location.column()
                    && location.file() == unexpected_location.file()
                {
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
                        .write_line_file(location.column, location.file())?
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

impl Error for GeneratorError<'_> {}

type Variables = HashMap<String, Variable>;

pub struct Generator {
    output: String,
    callable_procs: Vec<ProcedureHeader>,
}

struct ProcGenerator<'a> {
    stack_loc: usize,
    variables: Variables,
    body: String,
    constants: String,
    callable_procs: &'a Vec<ProcedureHeader>,
}

#[derive(Debug, Clone)]
pub struct ProcedureHeader {
    pub callable_path: Path,
    pub real_path: Path,
    pub parameters: Vec<WithLocation<Type>>,
    pub return_type: Type,
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
        importable_procs: &'a Vec<ProcedureHeader>,
    ) -> Result<String, GeneratorError<'a>> {
        for procedure in ast.procedures.iter() {
            let param_type = procedure
                .parameters
                .iter()
                .map(|e| e.param_type.clone())
                .collect();
            self.callable_procs.push(ProcedureHeader {
                real_path: package_path.clone().join(Path::new(&procedure.name.value)),
                callable_path: Path::new(&procedure.name.value),
                parameters: param_type,
                return_type: *procedure.return_type,
            });
        }

        self.callable_procs.extend(importable_procs.clone());

        let mut imported_procs = Vec::new();
        for import in &ast.imports {
            let mut exists = false;
            self.callable_procs.iter().for_each(|procs| {
                if import.path.path.len() > procs.callable_path.path.len() {
                    return;
                }

                if procs
                    .callable_path
                    .path
                    .iter()
                    .enumerate()
                    .all(|(i, e)| import.path.path.get(i).is_none_or(|ee| ee == e))
                {
                    imported_procs.push(ProcedureHeader {
                        callable_path: procs
                            .callable_path
                            .clone()
                            .import(import.path.value.clone()),
                        real_path: procs.real_path.clone(),
                        parameters: procs.parameters.clone(),
                        return_type: procs.return_type,
                    });
                    exists = true;
                }
            });

            if !exists {
                return Err(GeneratorError::UndefinedModule(&import.path));
            }
        }

        self.callable_procs.extend(imported_procs);

        for (procedure, header) in ast.procedures.iter().zip(self.callable_procs.iter()) {
            let proc_gen = ProcGenerator::new(&self.callable_procs);
            self.output.push_str(&proc_gen.gen_proc(procedure, header)?);
        }
        return Ok(self.output);
    }
}

impl<'a> ProcGenerator<'a> {
    fn new(callable_procs: &'a Vec<ProcedureHeader>) -> Self {
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
        header: &ProcedureHeader,
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
