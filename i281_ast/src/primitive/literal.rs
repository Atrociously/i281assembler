use i281_core::TokenIter;

use crate::{error::Error, punct, type_enum, ParseItem, Result, util};

type_enum!(Literal {
    Byte(u8),
    Array(Vec<Literal>),
    NotSet,
});

impl ParseItem for Byte {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let lit = input.next().ok_or(Error::InvalidLiteral)?;
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

impl ParseItem for Array {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let first = match util::parse_either::<Byte, NotSet, _>(input)? {
            util::Either::Left(byte) => Literal::Byte(byte),
            util::Either::Right(ns) => Literal::NotSet(ns),
        };

        let mut data = vec![first];
        let mut peeked = input.peek().map(str::chars).ok_or(Error::InvalidLiteral)?;
        while let Ok(_) = <punct::Comma as crate::Parse>::parse(&mut peeked) {
            let _next_comma = input.next(); // consume comma from input
            
            let next = match util::parse_either::<Byte, NotSet, _>(input)? {
                util::Either::Left(byte) => Literal::Byte(byte),
                util::Either::Right(ns) => Literal::NotSet(ns),
            };
            data.push(next);

            peeked = match input.peek() {
                Some(s) => s.chars(),
                None => return Ok(Self(data)),
            };
        }
        Ok(Self(data))
    }
}

impl ParseItem for NotSet {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let tok = input.next().ok_or(Error::InvalidLiteral)?;
        if tok.len() != 1 {
            Err(Error::InvalidLiteral.into())
        } else if tok.chars().next().unwrap() == punct::Question::REPR {
            Ok(NotSet)
        } else {
            Err(Error::InvalidLiteral.into())
        }
    }
}

impl ParseItem for Literal {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let next = input.next().ok_or(Error::InvalidLiteral)?;
        let mut next = next.chars();
        // try to parse a byte
        match <Byte as crate::Parse>::parse(&mut next.clone()) {
            Ok(byte) => {
                let peeked_char = input.peek().map(str::chars).and_then(|mut c| c.next());
                if peeked_char == Some(punct::Comma::REPR) {
                    let _comma = input.next(); // consume the peeked comma
                    let mut arr = Array::parse(input)?;
                    arr.0.insert(0, Self::Byte(byte));
                    Ok(Self::Array(arr))
                } else {
                    Ok(Self::Byte(byte))
                }
            }
            // byte/array failed try not set
            Err(..) => match <NotSet as crate::Parse>::parse(&mut next) {
                Ok(ns) => {
                    let peeked_char = input.peek().map(str::chars).and_then(|mut c| c.next());
                    if peeked_char == Some(punct::Comma::REPR) {
                        let _comma = input.next(); // consume the peeked comma
                        let mut arr = Array::parse(input)?;
                        arr.0.insert(0, Self::NotSet(ns));
                        Ok(Self::Array(arr))
                    } else {
                        Ok(Self::NotSet(ns))
                    }
                },
                // all possible options failed this is not a valid literal
                Err(..) => Err(Error::InvalidLiteral.into()),
            },
        }
    }
}
