use nom::{branch::alt, bytes::complete::tag, combinator::map};

use crate::ParseNom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    pub const ALL: &'static [&'static str] = &["A", "B", "C", "D"];
}

impl ParseNom for Register {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        alt((
            map(tag("A"), |_| Self::A),
            map(tag("B"), |_| Self::B),
            map(tag("C"), |_| Self::C),
            map(tag("D"), |_| Self::D),
        ))(input)
    }
}
