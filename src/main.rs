#![forbid(unsafe_code)]

use std::{path::PathBuf, ffi::OsString};

use clap::Parser;

use i281_ast::Root;
use miette::IntoDiagnostic;

#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
enum EmitKind {
    Ast,
    AsmIr,
    #[default]
    Verilog,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, value_enum, default_value_t)]
    emit: EmitKind,
    #[arg(long, short, default_value_os_t = PathBuf::from("./target/"))]
    out_dir: PathBuf,
    filename: OsString,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    let input = std::fs::read_to_string(args.filename).into_diagnostic()?;
    let input = input.as_str();

    let ast = Root::parse(input)?;
    match args.emit {
        EmitKind::Ast => {
            let output = std::io::stdout().lock();
            serde_json::to_writer(output, &ast).into_diagnostic()?;
        }
        EmitKind::AsmIr => {
            let output = std::io::stdout().lock();
            let mut err = std::io::stderr().lock();
            let ir = i281_compiler::analyze(&mut err, ast)?;
            serde_json::to_writer(output, &ir).into_diagnostic()?;
        }
        EmitKind::Verilog => {
            let mut output = std::io::stderr().lock();
            let ir = i281_compiler::analyze(&mut output, ast)?;
            i281_compiler::compile_verilog(args.out_dir, ir).into_diagnostic()?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;
    use std::env;
    use walkdir::WalkDir;

    use i281_ast::{ParseError, Root};

    #[test]
    fn test_examples() -> miette::Result<()> {
        let current_dir = env::current_dir().into_diagnostic()?;

        for entry in WalkDir::new(current_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|f| f.file_name().to_string_lossy().ends_with("asm"))
        {
            let path = entry.path();
            let input = std::fs::read_to_string(path).into_diagnostic()?;

            let _root = Root::parse(&input).map_err(ParseError::into_static)?;
        }
        Ok(())
    }
}
