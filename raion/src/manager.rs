use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

use crate::{compiler::rin_compiler::RinCompiler, lexer::rin_lexer::RinLexer};
use crate::{
    compiler::{asm_compiler::ASMCompiler, rin_compiler::Path as RinPath},
    lexer::asm_lexer::ASMLexer,
};
use common::sin::Sin;
use io::Write;

struct CompilerManager<'a, 'b> {
    working_dir: &'b Path,
    package_name: &'a str,
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
            package_name,
        }
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
        let lexer = RinLexer::new(&self.buffer, self.source_file);
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
        let mut compiler = RinCompiler::new(tokens, self.module_path);
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
        let generated_asm = match compiler.generate() {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{err}");
                return Err(format!(
                    "could not compile `{}` due to previous generation error",
                    self.package_name
                ));
            }
        };

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
        let lexer = ASMLexer::new(&generated_asm, self.source_file);
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
