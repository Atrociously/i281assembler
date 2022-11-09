use nom::{branch::alt, bytes::complete::tag, combinator::map};

use crate::ParseNom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Oper {
    Add,
    Sub,
}

impl ParseNom for Oper {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        alt((map(tag("+"), |_| Self::Add), map(tag("-"), |_| Self::Sub)))(input)
    }
}
