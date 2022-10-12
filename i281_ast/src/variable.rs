use i281_core::TokenIter;

use crate::{keyword, Ident, Literal, ParseItem, util::parse_sep, Result};

#[derive(Clone, Debug)]
pub struct Variable {
    pub ident: Ident,
    pub value: Literal,
}

impl ParseItem for Variable {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let (ident, value) = parse_sep::<Ident, keyword::Byte, Literal, I>(input)?;
        Ok(Variable { ident, value })
    }
}
