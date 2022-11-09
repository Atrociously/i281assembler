use nom::{branch::permutation, combinator::opt};

use crate::{directive, sealed::ParseNom, ParseError, Span};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Root {
    pub data: Option<directive::Data>,
    pub code: directive::Code,
}

impl Root {
    pub fn parse<'a>(input: &'a str) -> Result<Self, ParseError<'a>> {
        <Self as ParseNom>::parse(Span::new(input))
            .map(|(_, out)| out)
            .map_err(|err| match err {
                nom::Err::Failure(e)
                | nom::Err::Error(e) => e,
                nom::Err::Incomplete(_) => unreachable!() // we use complete in all parsers
            })
    }
}

impl ParseNom for Root {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, (data, code)) =
            permutation((opt(directive::Data::parse), directive::Code::parse))(input)?;
        Ok((input, Self { data, code }))
    }
}
