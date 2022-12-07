use nom::{bytes::complete::take_till1, character::is_alphanumeric};

use crate::{ParseError, ParseNom};

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

fn non_alphanum(c: char) -> bool {
    if c.is_ascii() {
        !is_alphanumeric(c as u8)
    } else {
        false
    }
}

impl ParseNom for Register {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, c) = take_till1(non_alphanum)(input)?;
        match c.to_uppercase().as_str() {
            "A" => Ok((input, Self::A)),
            "B" => Ok((input, Self::B)),
            "C" => Ok((input, Self::C)),
            "D" => Ok((input, Self::D)),
            _ => Err(nom::Err::Error(ParseError::new(
                input,
                nom::error::ErrorKind::Alt,
            ))),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Register;
    use crate::Parse;

    #[test]
    fn register1() {
        assert_eq!(Register::parse("a").unwrap().1, Register::A);
        assert_eq!(Register::parse("A").unwrap().1, Register::A);
    }

    #[test]
    fn register2() {
        assert_eq!(Register::parse("b").unwrap().1, Register::B);
        assert_eq!(Register::parse("B").unwrap().1, Register::B);
    }

    #[test]
    fn register3() {
        assert_eq!(Register::parse("c").unwrap().1, Register::C);
        assert_eq!(Register::parse("C").unwrap().1, Register::C);
    }

    #[test]
    fn register4() {
        assert_eq!(Register::parse("d").unwrap().1, Register::D);
        assert_eq!(Register::parse("D").unwrap().1, Register::D);
    }

    #[test]
    #[should_panic]
    fn register5() {
        Register::parse("domeasolid").unwrap();
    }
}
