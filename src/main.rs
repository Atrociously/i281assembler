use std::{fs::File, io::BufReader};

use clap::Parser;
use color_eyre::Result;
mod compiler;
mod error;
mod static_analysis;

use i281_ast::Root;
use i281_core::BufReadCharsExt;

use crate::compiler::compile_ast;

#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
enum OutputKind {
    JsonAst,
    DebugInfo,
    #[default]
    HwVerilog,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_enum, default_value_t)]
    output_kind: OutputKind,
    filename: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let file = File::open(args.filename)?;
    let mut reader = BufReader::new(file);
    let mut chars = reader.chars().map(|c| c.expect("chars failed"));

    let root = Root::parse(&mut chars)?;

    match args.output_kind {
        OutputKind::JsonAst => {
            println!("{}", serde_json::to_string(&root)?);
        },
        OutputKind::DebugInfo => {
            dbg!(root);
        },
        OutputKind::HwVerilog => {
            compile_ast(root)
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs::File, io::BufReader};
    use walkdir::WalkDir;
    use color_eyre::Result;

    use i281_core::BufReadCharsExt;
    use i281_ast::Root;

    #[test]
    fn test_examples() -> Result<()> {
        color_eyre::install()?;
        let current_dir = env::current_dir()?;

        for entry in WalkDir::new(current_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|f| f.file_name().to_string_lossy().ends_with("asm"))
        {
            let path = entry.path();
            let file = File::open(&path)?;
            let mut reader = BufReader::new(file);
            let mut chars = reader.chars().map(|c| c.expect("chars failed"));

            print!("File: {:?} ", path);
            let root = Root::parse(&mut chars);
            if root.is_ok() {
                println!("SUCCESS");
            } else {
                println!("FAILURE");
            }
            //dbg!(root);
        }
        Ok(())
    }
}
