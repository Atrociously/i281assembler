use std::borrow::Cow;

use miette::{Diagnostic, SourceSpan};
use nom::error::ErrorKind;
use thiserror::Error;

use crate::Span;

#[derive(Clone, Error, Debug, Diagnostic)]
#[error("Failure to parse input on line: {line_number}")]
#[diagnostic(code(ast::parse_error), help("double check your syntax"))]
pub struct ParseError<'b> {
    #[source_code]
    input: Cow<'b, str>,
    line_number: u32,
    #[label("While parsing this")]
    err_span: SourceSpan,
    kind: ErrorKind,
}

impl<'a> ParseError<'a> {
    pub fn new(input: Span<'a>, kind: ErrorKind) -> Self {
        <Self as nom::error::ParseError<Span>>::from_error_kind(input, kind)
    }

    pub fn into_static(self) -> ParseError<'static> {
        let input = Cow::Owned(self.input.into_owned());
        let line_number = self.line_number;
        let err_span = self.err_span;
        let kind = self.kind;
        ParseError {
            input,
            line_number,
            err_span,
            kind,
        }
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        let line_number = input.location_line();
        let end = input.location_offset();
        let start = end;
        //let start = input.extra[..end].rfind(' ').unwrap_or(0);
        let end = end - start;
        Self {
            input: Cow::Borrowed(input.extra),
            line_number,
            err_span: SourceSpan::new(start.into(), end.into()),
            kind,
        }
    }

    fn append(_input: Span, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}
