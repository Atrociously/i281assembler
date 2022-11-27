use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0_count,
    sequence::pair,
};

use crate::ParseNom;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Ident(String);

impl Ident {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl<'a> From<&'a str> for Ident {
    fn from(s: &'a str) -> Self {
        Self(s.to_owned())
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ParseNom for Ident {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, ident) = recognize(pair(
            alt((alpha1, tag("_"))), // letters or underscore for start letter
            many0_count(alt((alphanumeric1, tag("_")))), // letters nums and undserscores within
        ))(input)?;
        Ok((input, Self(ident.to_string())))
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::Ident;
    use crate::Parse;

    #[test]
    fn ident1() {
        Ident::parse("a1").unwrap();
    }

    #[test]
    #[should_panic]
    fn ident2() {
        Ident::parse("1a").unwrap();
    }

    #[test]
    #[should_panic]
    fn ident3() {
        Ident::parse("  ab").unwrap();
    }

    #[test]
    fn ident4() {
        Ident::parse("ab   ").unwrap();
    }
}
