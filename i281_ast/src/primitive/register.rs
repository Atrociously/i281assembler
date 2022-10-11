use crate::{Parse, error::Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Parse for Register {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let name: String = input.take_while(|c| !c.is_whitespace()).collect();
        
        match name.to_uppercase().as_str() {
            "A" => Ok(Register::A),
            "B" => Ok(Register::B),
            "C" => Ok(Register::C),
            "D" => Ok(Register::D),
            _ => Err(Error::InvalidRegister),
        }
    }
}
