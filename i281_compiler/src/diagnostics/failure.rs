use i281_ast::{Label, ParseError, Variable};

#[derive(Clone, Debug, miette::Diagnostic, thiserror::Error)]
pub enum Failure {
    #[error("{0}")]
    Parse(ParseError<'static>),
    // these are failures because I am unsure how the compiler should resolve name collisions beyond
    // immediatly spitting out an error
    #[error("variable with name `{}` is defined more than once", .0.ident)]
    NonUniqueVariable(Variable),
    #[error("label with name `{}` is defined more than once", .0.ident)]
    NonUniqueLabel(Label),
    #[error("")]
    Skip, // signal variant
}

impl Failure {
    /// Returns `true` if the failure is [`Skip`].
    ///
    /// [`Skip`]: Failure::Skip
    #[must_use]
    pub fn is_skip(&self) -> bool {
        matches!(self, Self::Skip)
    }
}

impl<'a> From<ParseError<'a>> for Failure {
    fn from(err: ParseError<'a>) -> Self {
        Self::Parse(err.into_static())
    }
}
