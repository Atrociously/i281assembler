use std::{fs::File, io::BufReader};

use clap::Parser;
use color_eyre::Result;
use utf8_chars::BufReadCharsExt;

use i281_ast::{Parse, Root};

#[derive(Parser, Debug)]
struct Args {
    filename: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let file = File::open(args.filename)?;
    let mut reader = BufReader::new(file);
    let mut chars = reader.chars().map(|c| c.expect("chars failed"));

    let root = Root::parse(&mut chars)?;
    dbg!(root);
    //let mut input = TokenIter::new_with(
    //    chars,
    //    |c| Punct::is_punct(c),
    //    ';',
    //    |c| c.is_whitespace() || c.is_control()
    //);

    //while let Some(token) = input.next() {
    //    dbg!(token);
    //}

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, fs::File, io::BufReader};
    use walkdir::WalkDir;
    use color_eyre::Result;

    use utf8_chars::BufReadCharsExt;

    use i281_ast::{Parse, Root};

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

            println!("File: {:?}", path);
            let root = Root::parse(&mut chars)?;
            dbg!(root);
        }
        Ok(())
    }
}
