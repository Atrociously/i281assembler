use crate::{Parse, error::Error, punct};

use super::Ident;

pub struct Label {
    pub ident: Ident,
}

impl Parse for Label {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let ident = Ident::parse(input)?;
        let _colon = punct::Colon::parse(input)?;
        Ok(Self {
            ident
        })
    }
}
