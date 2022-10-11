use crate::{Parse, error::Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oper {
    Add,
    Sub,
}

impl Parse for Oper {
    type Err = Error;
    
    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        match input.next() {
            Some('+') => Ok(Self::Add),
            Some('-') => Ok(Self::Sub),
            _ => Err(Error::InvalidOper)
        }
    }
}
