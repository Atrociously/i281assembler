use crate::{error::Error, Parse, punct, Instruction, Variable, Label};

use super::Ident;

/// The main organizational block of assembly code
/// a directive reads all instructions after it until it reaches another directive
pub struct Directive {
    pub ident: Ident,
    pub variables: Vec<Variable>,
    pub labels: Vec<(Label, usize)>,
    pub instructions: Vec<Instruction>,
}

impl Parse for Directive {
    type Err = Error;
    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let _dot = punct::Dot::parse(input)?;
        let ident = Ident::parse(input)?;
        Ok(Self {
            ident,
            variables: vec![],
            labels: vec![],
            instructions: vec![],
        })
    }
}
