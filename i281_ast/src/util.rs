use color_eyre::Result;

use i281_core::TokenIter;

use crate::{ParseItem, error::Error};

pub(crate) fn parse_with_sep<A, S, B, I>(input: &mut TokenIter<I>) -> Result<(A, S, B)>
where
    A: ParseItem + std::fmt::Debug, // before seperator
    B: ParseItem + std::fmt::Debug, // after seperator
    S: ParseItem + std::fmt::Debug, // seperator
    I: Iterator<Item = char>, // chars
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
    I: Iterator<Item = char>, // chars
{
    parse_with_sep::<A, S, B, I>(input).map(|(a, _, b)| (a, b))
}

pub(crate) fn parse_surround<O, C, F, R, I>(input: &mut TokenIter<I>, mut f: F) -> Result<Vec<R>>
where
    F: FnMut(&mut TokenIter<I>) -> Result<R>,
    O: ParseItem + std::fmt::Debug,
    C: ParseItem + std::fmt::Debug,
    I: Iterator<Item = char>
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
            None => {
                return Err(Error::InvalidSurround.into())
            }
        }
    }
    let _close = C::parse(input)?;

    Ok(items)
}
