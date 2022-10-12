use crate::{error::Error, type_enum};

macro_rules! keyword {
    ($($variant:ident == $kw:literal),+ $(,)?) => {
        type_enum!(Keyword {
            $($variant),+
        });

        $(
        impl $crate::ParseItem for $variant {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let kw = input.next().ok_or($crate::Error::InvalidKeyword)?.to_uppercase();

                if kw == $kw {
                    Ok(Self)
                } else {
                    Err(Error::InvalidKeyword.into())
                }
            }
        }
        )+

        impl $crate::ParseItem for Keyword {
            fn parse<I: Iterator<Item = char>>(input: &mut ::i281_core::TokenIter<I>) -> $crate::Result<Self> {
                let kw = input.next().ok_or($crate::Error::InvalidKeyword)?.to_uppercase();

                match kw.as_str() {
                    $($kw => Ok(Self::$variant($variant)),)+
                    _ => Err(Error::InvalidKeyword.into())
                }
            }
        }
    }
}

keyword! {
    Byte == "BYTE",
}
