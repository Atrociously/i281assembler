use std::path::Path;

use i281_ast::{Instruction, ParseError, Root};

use crate::{
    analyzer::StaticAnalyzer,
    error::Error,
    writers::{CodeWriter, DataWriter},
};

pub struct Compiler<P: CompilePass> {
    inner: P,
}

mod sealed {
    pub trait Sealed {}
}

pub trait CompilePass: sealed::Sealed {}

pub struct Parse;
impl sealed::Sealed for Parse {}
impl CompilePass for Parse {}

pub struct Analyze {
    ast: Root,
}
impl sealed::Sealed for Analyze {}
impl CompilePass for Analyze {}

pub struct Output {
    ast: Root,
}
impl sealed::Sealed for Output {}
impl CompilePass for Output {}

impl Compiler<Parse> {
    pub fn new() -> Self {
        Self { inner: Parse }
    }

    pub fn parse<'a>(&self, input: &'a str) -> Result<Compiler<Analyze>, ParseError<'static>> {
        let root = Root::parse(input).map_err(ParseError::into_static)?;

        Ok(Compiler {
            inner: Analyze { ast: root },
        })
    }
}

impl Compiler<Analyze> {
    pub fn ast(self) -> Root {
        self.inner.ast
    }

    pub fn analyze(self) -> Result<Compiler<Output>, Error> {
        let ast = self.ast();

        let variables = ast
            .data
            .as_ref()
            .map(|v| v.variables.as_slice())
            .unwrap_or(&[]);
        let labels = ast.code.labels.as_slice();

        let analyzer = StaticAnalyzer::new(variables, labels);

        for instruction in ast.code.instructions.iter() {
            match instruction {
                Instruction::InputC(addr) => {
                    analyzer.check_address_constness(addr)?;
                }
                Instruction::InputCF(addr) => {
                    analyzer.check_address_validity(addr)?;
                }
                Instruction::InputD(addr) => {
                    analyzer.check_address_constness(addr)?;
                }
                Instruction::InputDF(addr) => {
                    analyzer.check_address_validity(addr)?;
                }
                Instruction::LoadP(_rx, ptr) => {
                    analyzer.check_pointer_validity(ptr)?;
                }
                Instruction::Load(_rx, addr) => {
                    analyzer.check_address_constness(addr)?;
                }
                Instruction::LoadF(_rx, addr) => {
                    analyzer.check_address_validity(addr)?;
                }
                Instruction::Store(addr, _rx) => {
                    analyzer.check_address_constness(addr)?;
                }
                Instruction::StoreF(addr, _rx) => {
                    analyzer.check_address_validity(addr)?;
                }
                _ => continue,
            }
        }

        Ok(Compiler {
            inner: Output { ast },
        })
    }
}

impl Compiler<Output> {
    fn ast(self) -> Root {
        self.inner.ast
    }

    pub fn write<P, W>(self, out_dir: P) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
        W: DataWriter + CodeWriter + Default,
    {
        let ast = self.ast();

        let mut out = std::io::stdout();
        let mut writer = W::default();

        for var in ast.data.map(|d| d.variables).unwrap_or_else(Vec::new) {
            writer.write_var(&mut out, var)?;
        }

        for ins in ast.code.instructions {
            writer.write_ins(&mut out, ins)?;
        }
        Ok(())
    }
}
