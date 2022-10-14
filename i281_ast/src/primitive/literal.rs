use i281_core::TokenIter;

use crate::{error::ErrorCode, punct, type_enum, ParseItem, Result, util};

type_enum!(Literal {
    Byte(u8),
    Array(Vec<Literal>),
    NotSet,
});

impl ParseItem for Byte {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let lit = input.next().ok_or(ErrorCode::unexpected_end("literal", input))?;
        let num = if lit.starts_with("0b") {
            u8::from_str_radix(&lit[2..], 2).map_err(|e| ErrorCode::LiteralInvalid.into_err(format!("{:?}", e), input))
        } else if lit.starts_with("0x") {
            u8::from_str_radix(&lit[2..], 16).map_err(|e| ErrorCode::LiteralInvalid.into_err(format!("{:?}", e), input))
        } else {
            u8::from_str_radix(&lit, 10).map_err(|e| ErrorCode::LiteralInvalid.into_err(format!("{:?}", e), input))
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
        let mut peeked = match input.peek().map(str::chars) {
            Some(s) => s,
            None => return Err(ErrorCode::unexpected_end("array", input))
        };
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
        let tok = input.next().ok_or(ErrorCode::unexpected_end("notset", input))?;
        // safe because tokens are guaranteed not to be length 0
        if tok.chars().next().unwrap() == punct::Question::REPR {
            Ok(NotSet)
        } else {
            Err(ErrorCode::LiteralInvalid.invalid_token(tok, punct::Question::REPR, input))
        }
    }
}

impl ParseItem for Literal {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let next = input.next().ok_or(ErrorCode::unexpected_end("literal", input))?;
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
                Err(..) => {
                    Err(ErrorCode::LiteralInvalid.into_err("expected literal to be either a byte, array, or not set but none found", input))
                },
            },
        }
    }
}
