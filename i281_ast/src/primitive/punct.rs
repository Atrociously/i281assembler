use crate::{type_enum, error::Error, Parse};

macro_rules! punct {
    ($($variant:ident == $char:literal),+ $(,)?) => {
        type_enum!(Punct {
            $($variant),+
        });

        $(impl Parse for $variant {
            type Err = Error;

            fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
                match input.next() {
                    Some($char) => Ok(Self),
                    _ => Err(Error::InvalidPunct),
                }
            }
        })+

        impl Parse for Punct {
            type Err = Error;

            fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
                match input.next() {
                    $(Some($char) => Ok(Self::$variant($variant)),)+
                    _ => Err(Error::InvalidPunct)
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
    Eq == '=',
}

