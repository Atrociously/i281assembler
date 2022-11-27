use nom::bytes::complete::tag;

use crate::ParseNom;

use super::Ident;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Label {
    pub ident: Ident,
    pub code_addr: usize,
}

impl ParseNom for Label {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, ident) = Ident::parse(input)?;
        let (input, _) = tag(":")(input)?;
        Ok((
            input,
            Self {
                ident,
                code_addr: 0,
            },
        ))
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.ident)
    }
}

#[cfg(test)]
mod test {
    use super::Label;
    use crate::Parse;

    #[test]
    fn label1() {
        let expected = Label {
            ident: "lab".into(),
            code_addr: 0,
        };
        assert_eq!(Label::parse("lab:").unwrap().1, expected);
    }

    #[test]
    #[should_panic]
    fn label2() {
        Label::parse("lab").unwrap();
    }
}
