use i281_core::TokenIter;

use crate::{error::ErrorCode, punct, ParseItem, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Oper {
    Add,
    Sub,
}

impl ParseItem for Oper {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let next = input.next().ok_or(ErrorCode::unexpected_end("operator", input))?;
        // safe because tokens are guaranteed size > 0
        match next.chars().next() {
            Some(punct::Add::REPR) => Ok(Self::Add),
            Some(punct::Sub::REPR) => Ok(Self::Sub),
            _ => Err(ErrorCode::OperInvalid.expected_one_of(next, ["+", "-"], input)),
        }
    }
}
