use nom::sequence::separated_pair;

use crate::{keyword, util::ws_end1, Ident, Literal, ParseNom};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Variable {
    pub ident: Ident,
    pub value: Literal,
    pub data_addr: usize,
}

impl ParseNom for Variable {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, (ident, value)) = separated_pair(
            ws_end1(Ident::parse),
            ws_end1(keyword::Byte::parse),
            Literal::parse,
        )(input)?;
        Ok((
            input,
            Variable {
                ident,
                value,
                data_addr: 0,
            },
        ))
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { ident, value, .. } = self;
        write!(f, "{ident} {} {value}", keyword::Byte)
    }
}

#[cfg(test)]
mod test {
    use super::Variable;
    use crate::{literal::Byte, Parse};

    #[test]
    fn variable1() {
        let expected = Variable {
            ident: "a".into(),
            value: Byte(10).into(),
            data_addr: 0,
        };
        assert_eq!(Variable::parse("a BYTE 10").unwrap().1, expected);
    }

    #[test]
    #[should_panic]
    fn variable2() {
        Variable::parse("a 10").unwrap(); // missing keyword
    }
}
