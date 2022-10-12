use crate::{type_enum, ParseItem};

macro_rules! punct {
    ($($variant:ident == $char:literal),+ $(,)?) => {
        type_enum!(Punct {
            $($variant),+
        });

        $(impl $variant {
            pub const REPR: char = $char;
        })+

        $(impl $crate::ParseItem for $variant {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let next = input.next().ok_or($crate::Error::InvalidPunct)?;
                if next.len() != 1 {
                    Err($crate::Error::InvalidPunct.into())
                } else if next.chars().next().unwrap() == $char {
                    Ok(Self)
                } else {
                    Err($crate::Error::InvalidPunct.into())
                }
            }
        })+

        impl Punct {
            pub const fn is_punct(c: char) -> bool {
                match c {
                    $($char => true,)+
                    _ => false,
                }
            }
        }

        impl ParseItem for Punct {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let next = input.next().ok_or($crate::Error::InvalidPunct)?;
                if next.len() != 1 {
                    Err($crate::Error::InvalidPunct.into())
                } else {
                    match next.chars().next().unwrap() {
                        $($char => Ok(Self::$variant($variant)),)+
                        _ => Err($crate::Error::InvalidPunct.into())
                    }
                }
            }
        }
    }
}

punct! {
    Dot == '.',
    Comma == ',',
    Colon == ':',
    SemiColon == ';',
    OpenBracket == '[',
    CloseBracket == ']',
    OpenBrace == '{',
    CloseBrace == '}',
    Eq == '=',
    Question == '?',
    Add == '+',
    Sub == '-',
}
