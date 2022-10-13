#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidIdent,
}

impl ErrorCode {
    pub fn into_err<M>(self, msg: M, col: usize, line: usize) -> Error
    where
        M: Into<std::borrow::Cow<'static, str>>,
    {
        Error::new(self, msg, col, line)
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIdent => write!(f, "ast::invalid_ident"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub struct Error {
    code: ErrorCode,
    msg: std::borrow::Cow<'static, str>,
    col: usize,
    line: usize,
}

impl Error {
    pub fn new<M>(code: ErrorCode, msg: M, col: usize, line: usize) -> Self
    where
        M: Into<std::borrow::Cow<'static, str>>,
    {
        Self {
            code,
            msg: msg.into(),
            col,
            line
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parsing failure at col: {} on line: {} code: `{}`\n message: {}", self.col, self.line, self.code, self.msg)
    }
}
