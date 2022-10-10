use crate::{error::Error, Parse};

use super::Ident;

pub struct Directive {
    pub ident: Ident,
}

impl Parse for Directive {
    type Err = Error;
    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let _dot = input.next().ok_or(Error::InvalidDirective)?;
        let ident = Ident::parse(input)?;
        Ok(Self { ident })
    }
}
