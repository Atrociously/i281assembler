use std::io::BufRead;

use i281_ast::{Directive, Parse, Token};

pub struct Tokenizer<T> {
    input: T,
    tokens: Vec<Token>,
    index: usize,
}

impl<T: BufRead> Tokenizer<T> {
    pub fn new(input: T) -> Self {
        Self {
            input,
            tokens: vec![],
            index: usize::MAX,
        }
    }

    pub fn next(&mut self) -> Result<Option<&Token>, Box<dyn std::error::Error>> {
        let mut line = String::new();
        let mut chars = match self.input.read_line(&mut line) {
            // if read_line returns Ok(0) we have reached the end of the input stream so we will
            // continue returning tokens until we reach the end of the token buffer
            Ok(n) if n == 0 => {
                if self.index < self.tokens.len() {
                    self.index += 1;
                    return Ok(self.tokens.get(self.index));
                } else {
                    return Ok(None);
                }
            }
            // if any number besides 0 is returned then we parse the line
            Ok(_) => line.chars().skip_while(|c| c.is_whitespace()).peekable(),
            Err(e) => return Err(e.into()),
        };

        // match on the first non-whitespace char encountered
        match chars.peek() {
            // the whole line is a comment/empty skip it and move on
            Some(';') | None => return self.next(),
            // line has data
            Some(_) => {
                while let Some(_) = chars.peek() {
                    let next = Token::parse(&mut chars)?;
                    chars = chars.skip_while(|c| c.is_whitespace());
                }
            },
        }
        todo!()
    }
}
