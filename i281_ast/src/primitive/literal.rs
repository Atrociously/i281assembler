use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, one_of},
    combinator::{map, opt, recognize},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
};

use crate::{type_enum, util::ws0, IResult, ParseError, ParseNom, Span};

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
        let (input, (first, mut vals)) = separated_pair(
            alt((
                map(Byte::parse, Literal::Byte),
                map(NotSet::parse, Literal::NotSet),
            )),
            ws0(tag(",")),
            separated_list1(
                ws0(tag(",")),
                alt((
                    map(Byte::parse, Literal::Byte),
                    map(NotSet::parse, Literal::NotSet),
                )),
            ),
        )(input)?;
        vals.insert(0, first);
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

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: remember to change this to intersperse when that becomes stable
        let v = self
            .0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", v)
    }
}

impl std::fmt::Display for NotSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?")
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(b) => b.fmt(f),
            Self::Array(a) => a.fmt(f),
            Self::NotSet(n) => n.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Array, Byte, Literal, NotSet};
    use crate::Parse;

    #[test]
    fn literal1() {
        assert_eq!(Literal::parse("98").unwrap().1, Literal::Byte(Byte(98)));
    }

    #[test]
    #[should_panic]
    fn literal2() {
        Literal::parse("192837453").unwrap();
    }

    #[test]
    fn literal3() {
        let expected = Literal::Array(Array(vec![
            Literal::Byte(Byte(78)),
            Literal::Byte(Byte(20)),
        ]));
        assert_eq!(Literal::parse("78,20").unwrap().1, expected);
        assert_eq!(Literal::parse("78 , 20").unwrap().1, expected);
    }

    #[test]
    fn literal4() {
        assert_eq!(Literal::parse("?").unwrap().1, Literal::NotSet(NotSet));
    }

    #[test]
    fn literal5() {
        let expected = Literal::Array(Array(vec![
            Literal::Byte(Byte(20)),
            Literal::NotSet(NotSet),
        ]));
        assert_eq!(Literal::parse("20,?").unwrap().1, expected);
    }
}
