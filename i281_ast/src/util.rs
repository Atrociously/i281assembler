use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{line_ending, multispace0, multispace1},
    combinator::{opt, value},
    multi::{many0_count, many1_count},
    sequence::{delimited, pair, preceded, terminated},
};

use crate::Span;

pub(crate) fn ws0<I, O, F, E>(f: F) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    F: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

pub(crate) fn ws_end0<I, O, F, E>(f: F) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    F: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    terminated(f, multispace0)
}

pub(crate) fn ws_start0<I, O, F, E>(f: F) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    F: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    preceded(multispace0, f)
}

pub(crate) fn ws_end1<I, O, F, E>(f: F) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    F: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    terminated(f, multispace1)
}

fn comma_eol_comment<'a, E>(input: Span<'a>) -> nom::IResult<Span<'a>, (), E>
where
    E: nom::error::ParseError<Span<'a>>,
{
    value((), pair(tag(";"), opt(is_not("\n\r"))))(input)
}

pub(crate) fn ending1<'a, E>(input: Span<'a>) -> nom::IResult<Span<'a>, (), E>
where
    E: nom::error::ParseError<Span<'a>>,
{
    value(
        (),
        terminated(opt(ws_start0(comma_eol_comment)), line_ending),
    )(input)
}

pub(crate) fn many0_endings<'a, E>(input: Span<'a>) -> nom::IResult<Span<'a>, (), E>
where
    E: nom::error::ParseError<Span<'a>>,
{
    value((), many0_count(ending1))(input)
}

pub(crate) fn many1_endings<'a, E>(input: Span<'a>) -> nom::IResult<Span<'a>, (), E>
where
    E: nom::error::ParseError<Span<'a>>,
{
    value((), many1_count(ending1))(input)
}

pub(crate) fn always_fails<I, O, F, E>(mut f: F) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    F: nom::Parser<I, O, E>,
    E: nom::error::ParseError<I>,
{
    move |input| {
        f.parse(input).map_err(|e| match e {
            nom::Err::Error(e) | nom::Err::Failure(e) => nom::Err::Failure(e),
            _ => e,
        })
    }
}

macro_rules! type_enum {
    (@base $name:ident $(<$($lif:tt),+>)? {$($variant:ident $(<$($varlif:tt),+>)?),*}) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "serde", serde(tag = "kind", content = "value", rename_all = "snake_case"))]
        pub enum $name $(<$($lif),+>)? {
            $($variant($variant $(<$($varlif),+>)? )),*
        }

        $(
        impl From<$variant> for $name {
            fn from(v: $variant) -> Self {
                Self::$variant(v)
            }
        }
        )*
    };
    ($name:ident $(<$($lif:tt),+>)? {
        $($variant:ident $(<$($varlif:tt),+>)? $(($data:ty))?),*
        $(,)?
    }) => {
        $(
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub struct $variant $(<$($varlif),+>)? $((pub $data))?;
        )*

        type_enum!(@base $name $(<$($lif),+>)? {$($variant $(<$($varlif),+>)?),*});
    };
}
pub(crate) use type_enum;
