use std::{
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    compiler::rin_compiler::{
        generator::{Generator, ProcedureHeader},
        RinAst, RinCompiler,
    },
    lexer::rin_lexer::RinLexer,
};
use crate::{
    compiler::{asm_compiler::ASMCompiler, rin_compiler::Path as RinPath},
    lexer::asm_lexer::ASMLexer,
};
use common::sin::Sin;
use io::Write;

pub struct CompilerManager<'a, 'b> {
    working_dir: &'b Path,
    package_name: &'a str,
    forest: Vec<CompiledFile>,
    procedures: Vec<ProcedureHeader>,
    build_dir: PathBuf,
}

#[derive(Debug)]
struct CompiledFile {
    ast: RinAst,
    path: RinPath,
}

pub struct SingleUseCompiler<'a, 'b, 'c> {
    source_file: &'a Path,
    output: &'a Path,
    module_path: &'b RinPath,
    buffer: String,
    output_asm: Option<&'a Path>,
    package_name: &'c str,
}

impl<'a, 'b> CompilerManager<'a, 'b> {
    pub fn new(working_dir: &'b Path, package_name: &'a str) -> Self {
        Self {
            working_dir,
            build_dir: working_dir.join("build"),
            package_name,
            forest: Vec::new(),
            procedures: Vec::new(),
        }
    }

    fn compile_recursive(
        &self,
        dir: &Path,
        module_path: RinPath,
    ) -> Result<Vec<CompiledFile>, String> {
        let paths =
            fs::read_dir(dir).map_err(|e| format!("couldn't read {}: {e}", dir.display()))?;

        let mut forest = Vec::new();
        for path in paths {
            let path = Into::<Arc<Path>>::into(
                path.map_err(|e| format!("couldn't read {}: {e}", dir.display()))?
                    .path(),
            );
            if path.is_dir() {
                forest.extend(
                    self.compile_recursive(
                        &path,
                        module_path
                            .clone()
                            .join(RinPath::new(path.file_name().expect("").to_str().unwrap())),
                    )?,
                );
            }

            if path.extension().is_none_or(|e| e != "rin") {
                continue;
            }

            let mut buffer = String::new();
            File::open(path.as_ref())
                .map_err(|e| format!("couldn't open {}: {e}", path.display()))?
                .read_to_string(&mut buffer)
                .map_err(|e| format!("couldn't read {}: {e}", path.display()))?;
            let lexer = RinLexer::new(&buffer, path.clone());
            let tokens = lexer.tokenize().map_err(|err| {
                eprintln!("{err}");
                format!(
                    "could not compile `{}` due to previous lexer error",
                    self.package_name
                )
            })?;
            let mut compiler = RinCompiler::new(tokens);
            compiler.parse().map_err(|err| {
                eprintln!("{err}");
                format!(
                    "could not compile `{}` due to previous parsing error",
                    self.package_name
                )
            })?;
            forest.push(CompiledFile {
                ast: compiler.ast_owned(),
                path: module_path
                    .clone()
                    .join(RinPath::new(path.file_stem().expect("").to_str().unwrap())),
            });
        }
        return Ok(forest);
    }

    pub fn parse_files(&mut self) -> Result<(), String> {
        self.forest = self.compile_recursive(
            &self.working_dir.join("src"),
            RinPath::new(self.package_name),
        )?;
        self.procedures = self.collect_function()?;
        return Ok(());
    }

    pub fn generate_asm(&mut self) -> Result<String, String> {
        let mut output = String::new();
        for ast in &self.forest {
            let generator = Generator::new();
            let generated_asm = match generator.generate(&ast.ast, &ast.path, &self.procedures) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("{err}");
                    return Err(format!(
                        "could not compile `{}` due to previous generation error",
                        self.package_name
                    ));
                }
            };
            output.push_str(&generated_asm);
        }

        let entry = &self
            .procedures
            .iter()
            .find(|e| e.real_path == RinPath::new(format!("{}.main.main", self.package_name)))
            .ok_or(format!("Can't find main procedure"))?
            .real_path;

        output.push_str(&format!(
            "proc start -> {{\n   call {}\n   exit a64\n}}\n",
            entry.parse()
        ));
        return Ok(output);
    }

    pub fn generate(&mut self) -> Result<(), String> {
        let generated_asm = self.generate_asm()?;
        let out_asm = self.build_dir.join("out.asm");
        if Path::new(&out_asm).exists() {
            fs::remove_file(&out_asm)
                .map_err(|e| format!("couldn't remove existing output assembly file: {e}"))?;
        }

        let mut file = File::create(&out_asm)
            .map_err(|e| format!("cannot create output assembly file: {e}"))?;
        write!(file, "{generated_asm}").map_err(|e| format!("cannot write to assembly: {e}"))?;
        let lexer = ASMLexer::new(&generated_asm, out_asm.into());
        let tokens = match lexer.tokenize() {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{err}");
                return Err(format!(
                    "could not compile `{}` due to previous byte code generation error",
                    self.package_name
                ));
            }
        };
        let compiler = ASMCompiler::new(tokens);
        let out_sin = self.build_dir.join(format!("{}.sin", self.package_name));
        match compiler.compile() {
            Ok((sections, data)) => {
                let sin = Sin::new(sections, &data).to_bytes();

                if Path::new(&out_sin).exists() {
                    fs::remove_file(&out_sin)
                        .map_err(|e| format!("couldn't remove existing sin file: {e}"))?;
                }
                File::create(&out_sin)
                    .map_err(|e| format!("cannot create sin file: {e}"))?
                    .write_all(&sin)
                    .map_err(|e| format!("cannot write to sin: {e}"))?;
            }
            Err(err) => {
                eprintln!("{err}");
                return Err(format!(
                    "could not compile `{}` due to previous byte code generation error",
                    self.package_name
                ));
            }
        };

        return Ok(());
    }

    pub fn collect_function(&mut self) -> Result<Vec<ProcedureHeader>, String> {
        let mut procedures = Vec::new();
        for ast in &self.forest {
            for proc in ast.ast.procedures_iter() {
                procedures.push(proc.to_header(ast.path.clone()));
            }
        }
        return Ok(procedures);
    }
}

impl<'a, 'b, 'c> SingleUseCompiler<'a, 'b, 'c> {
    pub fn new(
        source_file: &'a Path,
        output: &'a Path,
        output_asm: Option<&'a Path>,
        module_path: &'b RinPath,
        package_name: &'c str,
    ) -> Self {
        Self {
            source_file,
            module_path,
            output,
            output_asm,
            package_name,
            buffer: String::new(),
        }
    }

    pub fn prepare_buffer(&mut self) -> Result<(), io::Error> {
        let mut file = File::open(self.source_file)?;
        file.read_to_string(&mut self.buffer)?;
        return Ok(());
    }

    pub fn generate_asm(&mut self) -> Result<String, String> {
        let lexer = RinLexer::new(&self.buffer, self.source_file.into());
        let tokens = match lexer.tokenize() {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{err}");
                return Err(format!(
                    "could not compile `{}` due to previous lexer error",
                    self.package_name
                ));
            }
        };
        let mut compiler = RinCompiler::new(tokens);
        match compiler.parse() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{err}");
                return Err(format!(
                    "could not compile `{}` due to previous parsing error",
                    self.package_name
                ));
            }
        };
        let generator = Generator::new();
        let mut generated_asm =
            match generator.generate(compiler.ast(), self.module_path, &Vec::new()) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("{err}");
                    return Err(format!(
                        "could not compile `{}` due to previous generation error",
                        self.package_name
                    ));
                }
            };

        let location = compiler
            .ast()
            .procedures_iter()
            .find(|e| e.value.name() == "main")
            .ok_or(format!("Can't find main procedures"))?
            .value
            .to_header(RinPath::new(self.package_name))
            .real_path;

        generated_asm.push_str(&format!(
            "proc start -> {{\n   call {}\n   exit a64\n}}\n",
            location.parse()
        ));

        if let Some(out_asm) = self.output_asm {
            if Path::new(out_asm).exists() {
                fs::remove_file(out_asm)
                    .map_err(|e| format!("couldn't remove existing output assembly file: {e}"))?;
            }

            let mut file = File::create(out_asm)
                .map_err(|e| format!("cannot create output assembly file: {e}"))?;
            write!(file, "{generated_asm}")
                .map_err(|e| format!("cannot write to assembly: {e}"))?;
        }

        return Ok(generated_asm);
    }

    pub fn generate(&mut self) -> Result<(), String> {
        let generated_asm = self.generate_asm()?;
        let lexer = ASMLexer::new(&generated_asm, self.source_file.into());
        let tokens = match lexer.tokenize() {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{err}");
                return Err(format!(
                    "could not compile `{}` due to previous byte code generation error",
                    self.package_name
                ));
            }
        };
        let compiler = ASMCompiler::new(tokens);
        match compiler.compile() {
            Ok((sections, data)) => {
                let sin = Sin::new(sections, &data).to_bytes();

                if Path::new(self.output).exists() {
                    fs::remove_file(self.output)
                        .map_err(|e| format!("couldn't remove existing sin file: {e}"))?;
                }
                File::create(self.output)
                    .map_err(|e| format!("cannot create sin file: {e}"))?
                    .write_all(&sin)
                    .map_err(|e| format!("cannot write to sin: {e}"))?;
            }
            Err(err) => {
                eprintln!("{err}");
                return Err(format!(
                    "could not compile `{}` due to previous byte code generation error",
                    self.package_name
                ));
            }
        };

        return Ok(());
    }
}
