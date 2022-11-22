use nom::bytes::complete::tag;

use crate::ParseNom;

use super::Ident;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Label {
    pub ident: Ident,
    pub code_addr: usize,
}

impl ParseNom for Label {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, ident) = Ident::parse(input)?;
        let (input, _) = tag(":")(input)?;
        Ok((
            input,
            Self {
                ident,
                code_addr: 0,
            },
        ))
    }
}
