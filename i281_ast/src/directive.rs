use i281_core::TokenIter;

use crate::{error::Error, punct, Instruction, Label, ParseItem, Variable, Ident, Result, OpCode};

// the main organizational block of assembly code
// there are two kinds of directives data and code
#[derive(Clone, Debug)]
pub enum Directive {
    Data {
        variables: Vec<Variable>,
    },
    Code {
        labels: Vec<(Label, usize)>,
        instructions: Vec<Instruction>,
    }
}

impl ParseItem for Directive {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let _dot = punct::Dot::parse(input)?;
        let kind = Ident::parse(input)?;

        match kind.as_str() {
            "data" => {
                let mut variables = Vec::new();
                let mut peeked = match input.peek() {
                    Some(p) => p,
                    None => return Ok(Self::Data { variables }),
                }.chars();
                while <punct::Dot as crate::Parse>::parse(&mut peeked).is_err() {
                    // parse will consume input and affect the peeked value
                    variables.push(Variable::parse(input)?);
                    // update the peek value
                    peeked = match input.peek() {
                        Some(p) => p.chars(),
                        None => return Ok(Self::Data { variables }),
                    };
                }
                Ok(Self::Data { variables })
            },
            "code" => {
                let mut index: usize = 0;
                let mut labels = Vec::new();
                let mut instructions = Vec::new();
                
                let mut peeked = match input.peek() {
                    Some(p) => p,
                    None => return Ok(Self::Code { labels, instructions }),
                }.chars();
                while <punct::Dot as crate::Parse>::parse(&mut peeked.clone()).is_err() {
                    if <OpCode as crate::Parse>::parse(&mut peeked).is_ok() {
                        // opcode parsed from peeked value meaning this must be an instruction
                        // without a label
                        match Instruction::parse(input) {
                            Ok(instruction) => {
                                instructions.push(instruction);
                                index += 1;
                            },
                            Err(e) => return Err(e),
                        }
                    } else {
                        match Label::parse(input) {
                            Ok(label) => {
                                labels.push((label, index));
                            }
                            Err(e) => return Err(e),
                        }
                    }

                    // peek the next value
                    peeked = match input.peek() {
                        Some(p) => p.chars(),
                        // if we have no more instructions then we are done
                        None => return Ok(Self::Code { labels, instructions })
                    };
                }

                Ok(Self::Code { labels, instructions })
            },
            _ => Err(Error::InvalidDirective.into()),
        }
    }
}
