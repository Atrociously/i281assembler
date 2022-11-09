use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, separated_pair},
};

use crate::{literal, sealed::ParseNom, Ident};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Pointer {
    pub var: Ident,
    pub offset: Option<usize>,
}

impl ParseNom for Pointer {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, (var, offset)) = delimited(
            tag("{"),
            separated_pair(
                Ident::parse,
                tag("+"),
                map(opt(literal::Byte::parse), |o| o.map(|b| b.0 as usize)),
            ),
            tag("}"),
        )(input)?;
        Ok((input, Self { var, offset }))
    }
}
