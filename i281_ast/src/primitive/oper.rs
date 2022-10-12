use i281_core::TokenIter;

use crate::{error::Error, punct, ParseItem, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oper {
    Add,
    Sub,
}

impl ParseItem for Oper {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let next = input.next().ok_or(Error::InvalidOper)?;
        if next.len() != 1 {
            Err(Error::InvalidOper.into())
        } else {
            match next.chars().next() {
                Some(punct::Add::REPR) => Ok(Self::Add),
                Some(punct::Sub::REPR) => Ok(Self::Sub),
                _ => Err(Error::InvalidOper.into()),
            }
        }
    }
}
