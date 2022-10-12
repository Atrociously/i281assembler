use std::{fs::File, io::BufReader};

use utf8_chars::BufReadCharsExt;
use color_eyre::Result;
use clap::Parser;

use i281_ast::{Root, Parse};

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
