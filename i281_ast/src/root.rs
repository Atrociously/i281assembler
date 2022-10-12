use i281_core::TokenIter;

use crate::{Directive, ParseItem, Result};

#[derive(Clone, Debug)]
pub struct Root {
    pub directives: Vec<Directive>
}

impl ParseItem for Root {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let mut directives = Vec::new();
        while let Some(_) = input.peek() {
            directives.push(Directive::parse(input)?);
        }
        Ok(Self { directives })
    }
}
