use clap::Parser;

use i281_compiler::{writers::VerilogWriter, Compiler};
use miette::IntoDiagnostic;

#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
enum OutputKind {
    JsonAst,
    DebugInfo,
    #[default]
    Verilog,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_enum, default_value_t)]
    output_kind: OutputKind,
    filename: String,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    let input = std::fs::read_to_string(args.filename).into_diagnostic()?;
    let input = input.as_str();

    let compiler = Compiler::new().parse(input)?;

    match args.output_kind {
        OutputKind::JsonAst => {
            println!(
                "{}",
                serde_json::to_string(&compiler.ast()).into_diagnostic()?
            );
        }
        OutputKind::DebugInfo => {
            dbg!(compiler.ast());
        }
        OutputKind::Verilog => {
            compiler
                .analyze()
                .unwrap()
                .write::<_, VerilogWriter>("")
                .unwrap();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;
    use std::env;
    use walkdir::WalkDir;

    use i281_ast::{Root, ParseError};

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
