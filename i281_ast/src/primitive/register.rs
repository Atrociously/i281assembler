use i281_core::TokenIter;

use crate::{error::Error, ParseItem, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl ParseItem for Register {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let name = input.next().ok_or(Error::InvalidRegister)?;

        match name.to_uppercase().as_str() {
            "A" => Ok(Register::A),
            "B" => Ok(Register::B),
            "C" => Ok(Register::C),
            "D" => Ok(Register::D),
            _ => Err(Error::InvalidRegister.into()),
        }
    }
}
