use i281_core::TokenIter;

use crate::{type_enum, punct, Ident, Instruction, Label, OpCode, ParseItem, Result, Variable, ErrorCode};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Data {
    pub variables: Vec<Variable>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Code {
    pub labels: Vec<(Label, usize)>,
    pub instructions: Vec<Instruction>,
}

// the main organizational block of assembly code
// there are two kinds of directives data and code
type_enum!(@base Directive {
    Data,
    Code
});

impl Directive {
    /// Returns `true` if the directive is [`Data`].
    ///
    /// [`Data`]: Directive::Data
    #[must_use]
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data { .. })
    }

    /// Returns `true` if the directive is [`Code`].
    ///
    /// [`Code`]: Directive::Code
    #[must_use]
    pub fn is_code(&self) -> bool {
        matches!(self, Self::Code { .. })
    }

    pub fn as_data(&self) -> Option<&Data> {
        if let Self::Data(data) = self {
            Some(data)
        } else {
            None
        }
    }

    pub fn as_code(&self) -> Option<&Code> {
        if let Self::Code(code) = self {
            Some(code)
        } else {
            None
        }
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
                    None => return Ok(Self::Data(Data { variables })),
                }
                .chars();
                while <punct::Dot as crate::Parse>::parse(&mut peeked).is_err() {
                    // parse will consume input and affect the peeked value
                    variables.push(Variable::parse(input)?);
                    // update the peek value
                    peeked = match input.peek() {
                        Some(p) => p.chars(),
                        None => return Ok(Self::Data(Data { variables })),
                    };
                }
                Ok(Self::Data(Data{ variables }))
            }
            "code" => {
                let mut index: usize = 0;
                let mut labels = Vec::new();
                let mut instructions = Vec::new();

                let mut peeked = match input.peek() {
                    Some(p) => p,
                    None => {
                        return Ok(Self::Code(Code {
                            labels,
                            instructions,
                        }))
                    }
                }
                .chars();
                while <punct::Dot as crate::Parse>::parse(&mut peeked.clone()).is_err() {
                    // unwrap is ok because peek returned some
                    if <OpCode as crate::Parse>::parse(&mut peeked).is_ok() {
                        // opcode parsed from peeked value meaning this must be an instruction
                        // without a label
                        match Instruction::parse(input) {
                            Ok(instruction) => {
                                instructions.push(instruction);
                                index += 1;
                            }
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
                        None => {
                            return Ok(Self::Code(Code {
                                labels,
                                instructions,
                            }))
                        }
                    };
                }

                Ok(Self::Code(Code {
                    labels,
                    instructions,
                }))
            }
            _ => Err(ErrorCode::DirectiveInvalid.expected_one_of(kind.as_str(), ["data", "code"], input)),
        }
    }
}
