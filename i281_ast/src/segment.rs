use nom::{
    bytes::complete::tag,
    combinator::opt,
    multi::many1,
    sequence::{delimited, pair, preceded, terminated},
};

use crate::{
    keyword,
    util::{always_fails, many0_endings, ws0},
    IResult, Instruction, Label, ParseNom, Span, Variable,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DataSegment {
    pub variables: Vec<Variable>,
}

impl PartialEq for DataSegment {
    fn eq(&self, other: &Self) -> bool {
        self.variables
            .iter()
            .zip(other.variables.iter())
            .all(|(a, b)| a == b)
    }
}
impl Eq for DataSegment {}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CodeSegment {
    pub labels: Vec<Label>,
    pub instructions: Vec<Instruction>,
}

impl PartialEq for CodeSegment {
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
impl Eq for CodeSegment {}

impl ParseNom for DataSegment {
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

impl ParseNom for CodeSegment {
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

// TODO: implement comprehensive tests for both segments
