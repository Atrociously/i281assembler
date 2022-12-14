use nom::{branch::permutation, combinator::opt};

use crate::{CodeSegment, DataSegment, ParseError, ParseNom, Span};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Root {
    pub data: Option<DataSegment>,
    pub code: CodeSegment,
}

impl Root {
    pub fn parse(input: &str) -> Result<Self, ParseError<'static>> {
        <Self as ParseNom>::parse(Span::new_extra(input, input))
            .map(|(_, out)| out)
            .map_err(|err| match err {
                nom::Err::Failure(e) | nom::Err::Error(e) => e,
                nom::Err::Incomplete(_) => unreachable!(), // we use complete in all parsers
            })
            .map_err(ParseError::into_static)
    }
}

impl ParseNom for Root {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, (data, code)) =
            permutation((opt(DataSegment::parse), CodeSegment::parse))(input)?;
        Ok((input, Self { data, code }))
    }
}
