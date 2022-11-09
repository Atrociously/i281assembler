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
    }
}

keyword! {
    Byte == "byte",
    Code == "code",
    Data == "data",
}
