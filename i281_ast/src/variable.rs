use crate::{Ident, Literal, Parse, error::Error, keyword};

pub struct Variable {
    pub ident: Ident,
    pub value: Literal,
}

impl Parse for Variable {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let ident = Ident::parse(input)?;
        let mut input = input.skip_while(|c| c.is_whitespace());
        let _kw_byte = keyword::Byte::parse(&mut input)?;
        let mut input = input.skip_while(|c| c.is_whitespace());
        let value = Literal::parse(&mut input)?;
        Ok(Variable {
            ident,
            value,
        })
    }
}
