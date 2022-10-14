use crate::type_enum;

macro_rules! keyword {
    ($($variant:ident == $kw:literal),+ $(,)?) => {
        type_enum!(Keyword {
            $($variant),+
        });

        $(
        impl $crate::ParseItem for $variant {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let kw = input.next().ok_or($crate::ErrorCode::unexpected_end("keyword", input))?.to_uppercase();

                if kw == $kw {
                    Ok(Self)
                } else {
                    Err($crate::ErrorCode::KeywordInvalid.invalid_token(kw, $kw, input))
                }
            }
        }
        )+

        impl Keyword {
            pub const ALL: &'static [&'static str] = &[$($kw),+];
        }

        impl $crate::ParseItem for Keyword {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let kw = input.next().ok_or($crate::ErrorCode::unexpected_end("keyword", input))?.to_uppercase();

                match kw.as_str() {
                    $($kw => Ok(Self::$variant($variant)),)+
                    _ => Err($crate::ErrorCode::KeywordInvalid.expected_one_of(kw, Self::ALL, input))
                }
            }
        }
    }
}

keyword! {
    Byte == "BYTE",
}
