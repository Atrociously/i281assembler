use super::Result;

use i281_core::TokenIter;

use crate::{ParseItem, Parse, ErrorCode};

pub(crate) fn parse_with_sep<A, S, B, I>(input: &mut TokenIter<I>) -> Result<(A, S, B)>
where
    A: ParseItem + std::fmt::Debug, // before seperator
    B: ParseItem + std::fmt::Debug, // after seperator
    S: ParseItem + std::fmt::Debug, // seperator
    I: Iterator<Item = char>,       // chars
{
    let a = A::parse(input)?; // parse the first
    let sep = S::parse(input)?; // parse the seperator
    let b = B::parse(input)?; // parse the second

    Ok((a, sep, b))
}

pub(crate) fn parse_sep<A, S, B, I>(input: &mut TokenIter<I>) -> Result<(A, B)>
where
    A: ParseItem + std::fmt::Debug, // before seperator
    B: ParseItem + std::fmt::Debug, // after seperator
    S: ParseItem + std::fmt::Debug, // seperator
    I: Iterator<Item = char>,       // chars
{
    parse_with_sep::<A, S, B, I>(input).map(|(a, _, b)| (a, b))
}

pub(crate) fn parse_surround<O, C, F, R, I>(input: &mut TokenIter<I>, mut f: F) -> Result<Vec<R>>
where
    F: FnMut(&mut TokenIter<I>) -> Result<R>,
    O: ParseItem + std::fmt::Debug,
    C: ParseItem + std::fmt::Debug,
    I: Iterator<Item = char>,
{
    let _start = O::parse(input);
    let mut next = match input.peek() {
        Some(n) => n,
        None => return f(input).map(|v| vec![v]),
    };

    let mut items = Vec::new();
    while <C as crate::Parse>::parse(&mut next.chars()).is_err() {
        items.push(f(input)?);
        next = match input.peek() {
            Some(n) => n,
            None => return Err(ErrorCode::unexpected_end("surround", input)),
        }
    }
    let _close = C::parse(input)?;

    Ok(items)
}

pub(crate) enum Either<L, R> {
    Left(L),
    Right(R),
}

pub(crate) fn parse_either<A, B, I>(input: &mut TokenIter<I>) -> Result<Either<A, B>>
where
    A: ParseItem,
    B: ParseItem + std::fmt::Debug,
    I: Iterator<Item = char>,
{
    let mut peeked = match input.peek().map(str::chars) {
        Some(s) => s,
        None => return Err(ErrorCode::unexpected_end("either", input))
    };
    match <A as Parse>::parse(&mut peeked.clone()) {
        Ok(a) => {
            input.next(); // consume peeked
            Ok(Either::Left(a))
        },
        Err(e1) => match <B as Parse>::parse(&mut peeked) {
            Ok(b) => {
                input.next(); // consume peeked
                Ok(Either::Right(b))
            },
            Err(e2) => {
                let msg = format!("expected either A or B but both failed, A: {}, B: {}", e1, e2);
                Err(ErrorCode::ExpectedEither.into_err(msg, input))
            }
        }
    }
}
