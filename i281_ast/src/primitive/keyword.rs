use crate::{type_enum, Parse, error::Error};

macro_rules! keyword {
    ($($variant:ident == $kw:literal),+ $(,)?) => {
        type_enum!(Keyword {
            $($variant),+
        });

        $(impl Parse for $variant {
            type Err = Error;

            fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
                let kw = input.take_while(|c| !c.is_whitespace()).collect::<String>().to_uppercase();

                if kw == $kw {
                    Ok(Self)
                } else {
                    Err(Error::InvalidKeyword)
                }
            }
        })+

        impl Parse for Keyword {
            type Err = Error;

            fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
                let kw = input.take_while(|c| !c.is_whitespace()).collect::<String>().to_uppercase();

                match kw.as_str() {
                    $($kw => Ok(Self::$variant($variant)),)+
                    _ => Err(Error::InvalidKeyword)
                }
            }
        }
    }
}

keyword! {
    Byte == "BYTE",
}
