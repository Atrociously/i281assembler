use nom::{
    bytes::complete::tag,
    combinator::opt,
    sequence::{delimited, pair},
};

use crate::{literal::Byte, util::ws0, Ident, Oper, ParseNom};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Pointer {
    pub var: Ident,
    pub offset: Option<(Oper, Byte)>,
}

impl ParseNom for Pointer {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, (var, offset)) = delimited(
            tag("{"),
            pair(Ident::parse, opt(pair(ws0(Oper::parse), Byte::parse))),
            tag("}"),
        )(input)?;
        Ok((input, Self { var, offset }))
    }
}

impl std::fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}", self.var)?;
        if let Some((op, off)) = &self.offset {
            write!(f, " {op} {off}")?;
        }
        write!(f, "}}")
    }
}

// TODO: implement tests for pointers if they are going to stay in the assembly language
