use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, one_of},
    combinator::{map, opt, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, preceded},
};

use crate::{sealed::ParseNom, type_enum, util::ws0, IResult, ParseError, Span};

type_enum!(Literal {
    Byte(i8),
    Array(Vec<Literal>),
    NotSet,
});

impl Literal {
    /// Get the size of the literal in bytes
    pub fn size_of(&self) -> usize {
        match self {
            Self::Byte(..) | Self::NotSet(..) => 1,
            Self::Array(arr) => arr.0.len(),
        }
    }
}

fn parse_int_err<'a>(
    input: Span<'a>,
) -> impl FnOnce(std::num::ParseIntError) -> nom::Err<ParseError<'a>> {
    move |_| nom::Err::Failure(ParseError::new(input, nom::error::ErrorKind::TooLarge))
}

fn binary(input: Span) -> IResult<i8> {
    let inp = input.clone();

    let (input, num) =
        preceded(alt((tag("0b"), tag("0B"))), recognize(many1(one_of("01"))))(input)?;
    let num: i8 = i8::from_str_radix(&num, 2).map_err(parse_int_err(inp))?;
    Ok((input, num))
}

fn hexadecimal(input: Span) -> IResult<i8> {
    let inp = input.clone();

    let (input, num) = preceded(alt((tag("0x"), tag("0X"))), hex_digit1)(input)?;
    let num: i8 = i8::from_str_radix(&num, 16).map_err(parse_int_err(inp))?;
    Ok((input, num))
}

fn decimal(input: Span) -> IResult<i8> {
    let inp = input.clone();

    let (input, num) = recognize(pair(opt(alt((tag("-"), tag("+")))), digit1))(input)?;
    let num: i8 = i8::from_str_radix(&num, 10).map_err(parse_int_err(inp))?;
    Ok((input, num))
}

impl ParseNom for Byte {
    fn parse(input: Span) -> IResult<Self> {
        let (input, num) = alt((binary, hexadecimal, decimal))(input)?;
        Ok((input, Self(num)))
    }
}

impl ParseNom for Array {
    fn parse(input: Span) -> IResult<Self> {
        let (input, vals) = separated_list1(
            ws0(tag(",")),
            alt((
                map(<Byte as ParseNom>::parse, Literal::Byte),
                map(<NotSet as ParseNom>::parse, Literal::NotSet),
            )),
        )(input)?;
        Ok((input, Self(vals)))
    }
}

impl ParseNom for NotSet {
    fn parse(input: Span) -> IResult<Self> {
        tag("?")(input).map(|(input, _)| (input, Self))
    }
}

impl ParseNom for Literal {
    fn parse(input: Span) -> IResult<Self> {
        alt((
            map(Array::parse, Literal::Array),
            map(Byte::parse, Literal::Byte),
            map(NotSet::parse, Literal::NotSet),
        ))(input)
    }
}
