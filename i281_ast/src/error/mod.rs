use std::borrow::Cow;

use miette::{Diagnostic, SourceOffset, SourceSpan};
use nom::error::ErrorKind;
use thiserror::Error;

use crate::Span;

#[derive(Error, Debug, Diagnostic)]
#[error("Failure to parse input")]
pub struct ParseError<'a> {
    #[source_code]
    input: Cow<'a, str>,
    #[label = "Error occurred parsing this"]
    err_span: SourceSpan,
    kind: ErrorKind,
}

impl<'a> ParseError<'a> {
    pub fn new(input: Span<'a>, kind: ErrorKind) -> Self {
        <Self as nom::error::ParseError<Span>>::from_error_kind(input, kind)
    }

    pub fn into_static(self) -> ParseError<'static> {
        let input = Cow::Owned(self.input.into_owned());
        let err_span = self.err_span;
        let kind = self.kind;
        ParseError {
            input,
            err_span,
            kind,
        }
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        let input_str = input.to_string();
        let start = SourceOffset::from_location(
            &input_str,
            input.location_line() as usize,
            input.get_utf8_column(),
        );
        let end = start.clone();
        Self {
            input: Cow::Borrowed(input.as_ref()),
            err_span: SourceSpan::new(start, end),
            kind,
        }
    }

    fn append(_input: Span, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}
