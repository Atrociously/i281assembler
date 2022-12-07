use nom::{branch::alt, bytes::complete::tag, combinator::map};

use crate::ParseNom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Oper {
    Add,
    Sub,
}

impl ParseNom for Oper {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        alt((map(tag("+"), |_| Self::Add), map(tag("-"), |_| Self::Sub)))(input)
    }
}

impl std::fmt::Display for Oper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Oper::Add => write!(f, "+"),
            Oper::Sub => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Oper;
    use crate::Parse;

    #[test]
    fn oper1() {
        assert_eq!(Oper::parse("+").unwrap().1, Oper::Add);
    }

    #[test]
    fn oper2() {
        assert_eq!(Oper::parse("-").unwrap().1, Oper::Sub);
    }
}
