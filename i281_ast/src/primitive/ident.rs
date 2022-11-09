use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0_count,
    sequence::pair,
};

use crate::ParseNom;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Ident(String);

impl Ident {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ParseNom for Ident {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, ident) = recognize(pair(
            alt((alpha1, tag("_"))), // letters or underscore for start letter
            many0_count(alt((alphanumeric1, tag("_")))), // letters nums and undserscores within
        ))(input)?;
        Ok((input, Self(ident.to_string())))
    }
}
