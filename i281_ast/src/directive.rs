use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, pair, preceded, terminated},
};

use crate::{
    keyword, type_enum,
    util::{always_fails, many0_endings, ws0},
    IResult, Instruction, Label, ParseNom, Span, Variable,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Data {
    pub variables: Vec<Variable>,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.variables
            .iter()
            .zip(other.variables.iter())
            .all(|(a, b)| a == b)
    }
}
impl Eq for Data {}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Code {
    pub labels: Vec<Label>,
    pub instructions: Vec<Instruction>,
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        let labels = self
            .labels
            .iter()
            .zip(other.labels.iter())
            .all(|(a, b)| a == b);
        let instructions = self
            .instructions
            .iter()
            .zip(other.instructions.iter())
            .all(|(a, b)| a == b);
        labels && instructions
    }
}
impl Eq for Code {}

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

impl ParseNom for Data {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, mut variables) = preceded(
            delimited(
                many0_endings,
                preceded(tag("."), keyword::Data::parse),
                always_fails(many0_endings),
            ),
            always_fails(many1(terminated(ws0(Variable::parse), many0_endings))),
        )(input)?;

        let mut data_addr: usize = 0;
        for var in variables.iter_mut() {
            var.data_addr = data_addr;
            data_addr += var.value.size_of();
        }
        Ok((input, Self { variables }))
    }
}

impl ParseNom for Code {
    fn parse(input: Span) -> IResult<Self> {
        let (input, code) = preceded(
            delimited(
                many0_endings,
                preceded(tag("."), keyword::Code::parse),
                always_fails(many0_endings),
            ),
            always_fails(many1(pair(
                opt(terminated(ws0(Label::parse), many0_endings)),
                terminated(ws0(Instruction::parse), many0_endings),
            ))),
        )(input)?;

        let (labels, instructions): (Vec<_>, Vec<_>) = code.into_iter().unzip();

        let labels: Vec<_> = labels
            .into_iter()
            .enumerate()
            .filter_map(|(i, label)| {
                if let Some(mut label) = label {
                    label.code_addr = i;
                    Some(label)
                } else {
                    None
                }
            })
            .collect();

        Ok((
            input,
            Self {
                labels,
                instructions,
            },
        ))
    }
}

impl ParseNom for Directive {
    fn parse(input: Span) -> IResult<Self> {
        alt((map(Data::parse, Self::Data), map(Code::parse, Self::Code)))(input)
    }
}

// TODO implement comprehensive tests for both directives
