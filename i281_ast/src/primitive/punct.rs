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
                let next = input.next().ok_or($crate::ErrorCode::unexpected_end("punct", input))?;
                // safe because tokens are guaranteed size > 0
                if next.chars().next().unwrap() == $char {
                    Ok(Self)
                } else {
                    Err($crate::ErrorCode::PunctInvalid.invalid_token(next, $char, input))
                }
            }
        })+

        impl Punct {
            pub const ALL: &'static [char] = &[$($char),+];

            pub const fn is_punct(c: char) -> bool {
                match c {
                    $($char => true,)+
                    _ => false,
                }
            }
        }

        impl ParseItem for Punct {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let next = input.next().ok_or($crate::ErrorCode::unexpected_end("punct", input))?;
                // safe because tokens are guaranteed size > 0
                match next.chars().next().unwrap() {
                    $($char => Ok(Self::$variant($variant)),)+
                    _ => Err($crate::ErrorCode::PunctInvalid.expected_one_of(next, Self::ALL.into_iter().map(|c| c.to_string()), input))
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
