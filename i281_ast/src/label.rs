use i281_core::TokenIter;

use crate::{punct, ParseItem, Result};

use super::Ident;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Label {
    pub ident: Ident,
}

impl ParseItem for Label {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let ident = Ident::parse(input)?;
        // label must be immediatly followed by a colon
        let _colon = punct::Colon::parse(input)?;
        Ok(Self { ident })
    }
}
