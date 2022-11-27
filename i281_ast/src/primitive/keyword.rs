use crate::type_enum;

macro_rules! keyword {
    ($($variant:ident == $kw:literal),+ $(,)?) => {
        type_enum!(Keyword {
            $($variant),+
        });

        $(
        impl $crate::ParseNom for $variant {
            fn parse(input: crate::Span) -> crate::IResult<Self> {
                let (input, _) = nom::bytes::complete::tag_no_case($kw)(input)?;
                Ok((input, Self))
            }
        }

        impl std::fmt::Display for $variant {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $kw)
            }
        }
        )+

        impl Keyword {
            pub const ALL: &'static [&'static str] = &[$($kw),+];
        }

        impl $crate::ParseNom for Keyword {
            fn parse(input: crate::Span) -> crate::IResult<Self> {
                nom::branch::alt((
                    $(nom::combinator::map($variant::parse, Self::$variant)),+
                ))(input)
            }
        }

        impl std::fmt::Display for Keyword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(v) => v.fmt(f),)*
                }
            }
        }
    }
}

keyword! {
    Byte == "BYTE",
    Code == "code",
    Data == "data",
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    #[test]
    fn keyword1() {
        assert_eq!(Keyword::parse("byte").unwrap().1, Keyword::Byte(Byte));
        assert_eq!(Keyword::parse("BYTE").unwrap().1, Keyword::Byte(Byte));
        assert_eq!(Keyword::parse("ByTe").unwrap().1, Keyword::Byte(Byte));
    }

    #[test]
    fn keyword2() {
        Code::parse("code").unwrap();
    }

    #[test]
    #[should_panic]
    fn keyword3() {
        Data::parse(" data").unwrap();
    }
}
