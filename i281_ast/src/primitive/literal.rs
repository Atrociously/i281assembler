use crate::{error::Error, type_enum, Parse};

type_enum!(Literal {
    Byte(u8),
    ByteArray(Vec<u8>),
    NotSet,
});

impl Parse for Byte {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let lit: String = input.take_while(|c| !c.is_whitespace()).collect();
        let num = if lit.starts_with("0b") {
            u8::from_str_radix(&lit[2..], 2).map_err(|_| Error::InvalidLiteral)
        } else if lit.starts_with("0x") {
            u8::from_str_radix(&lit[2..], 16).map_err(|_| Error::InvalidLiteral)
        } else {
            u8::from_str_radix(&lit, 10).map_err(|_| Error::InvalidLiteral)
        }?;
        Ok(Self(num))
    }
}

impl Parse for ByteArray {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let text: String = input.collect(); // collect the rest of the line into a string
        let arr = text
            .split(',')
            .map(|s| Byte::parse(&mut s.trim().chars()).map(|b| b.0))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(arr))
    }
}

impl Parse for NotSet {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        match input.next() {
            Some('?') => { Ok(NotSet) },
            _ => { Err(Error::InvalidLiteral) }
        }
    }
}

impl Parse for Literal {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {

        match input.peekable().peek() {
            Some('?') => NotSet::parse(input).map(Self::NotSet),
            Some('0'..='9') => {
                let byte = Byte::parse(input)?;
                if input.skip_while(|c| c.is_whitespace()).next() == Some(',') {
                    ByteArray::parse(input).map(|mut v| {
                        // insert the first byte into the byte array
                        v.0.insert(0, byte.0);
                        Self::ByteArray(v)
                    })
                } else {
                    Ok(Self::Byte(byte))
                }
            }
            _ => Err(Error::InvalidLiteral)
        }
    }
}
