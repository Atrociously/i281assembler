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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Code {
    pub labels: Vec<Label>,
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

impl Data {
    fn parse_inner(input: Span) -> IResult<Self> {
        let (input, variables) = preceded(
            delimited(
                many0_endings,
                preceded(tag("."), keyword::Data::parse),
                always_fails(many0_endings),
            ),
            always_fails(many1(terminated(ws0(Variable::parse), many0_endings))),
        )(input)?;

        Ok((input, Self { variables }))
    }
}

impl ParseNom for Data {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let res = Self::parse_inner(input);
        Variable::reset();
        // this ensures that the variable memory offset will always be reset after
        // parsing a data directive
        res
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
