use i281_core::TokenIter;

use crate::{ErrorCode, ParseItem, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    pub const ALL: &'static [&'static str] = &["A", "B", "C", "D"];
}

impl ParseItem for Register {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let name = input.next().ok_or(ErrorCode::unexpected_end("register", input))?;
        match name.to_uppercase().as_str() {
            "A" => Ok(Register::A),
            "B" => Ok(Register::B),
            "C" => Ok(Register::C),
            "D" => Ok(Register::D),
            _ => Err(ErrorCode::RegisterInvalid.expected_one_of(name, Self::ALL, input)),
        }
    }
}
