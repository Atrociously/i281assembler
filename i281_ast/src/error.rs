use i281_core::TokenIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnexpectedEnd,
    AddressInvalid,
    AddressItemInvalid,
    IdentInvalidStart,
    IdentInvalidChar,
    PunctInvalid,
    KeywordInvalid,
    LiteralInvalid,
    ExpectedEither,
    OpCodeInvalid,
    OperInvalid,
    RegisterInvalid,
    DirectiveInvalid,
    RootInvalid,
}

impl ErrorCode {
    pub fn unexpected_end<E, I>(expected_type: E, input: &TokenIter<I>) -> Error
    where
        E: std::fmt::Display,
        I: Iterator<Item = char>,
    {
        let msg = format!("unexpected end of input expected an `{}`", expected_type);
        Self::UnexpectedEnd.into_err(msg, input)
    }

    pub fn expected_one_of<T, E, A, I>(self, invalid_token: T, expected: E, input: &TokenIter<I>) -> Error
    where
        T: std::fmt::Display,
        E: IntoIterator<Item = A>,
        A: AsRef<str>,
        I: Iterator<Item = char>,
    {
        let expected: String = expected.into_iter().fold(String::new(), |mut accum, e| {
            accum += e.as_ref();
            accum.push(',');
            accum
        });
        let msg = format!("invalid token: '{}' expected one of: [{}]", invalid_token, &expected[..expected.len()-1]);
        self.into_err(msg, input)
    }

    pub fn invalid_token<T, E, I>(self, invalid_token: T, expected: E, input: &TokenIter<I>) -> Error
    where
        T: std::fmt::Display,
        E: std::fmt::Display,
        I: Iterator<Item = char>,
    {
        let msg = format!("invalid token: '{}' expected: '{}'", invalid_token, expected);
        self.into_err(msg, input)
    }

    pub fn into_err<M, I>(self, msg: M, input: &TokenIter<I>) -> Error
    where
        M: Into<std::borrow::Cow<'static, str>>,
        I: Iterator<Item = char>,
    {
        Error::new(self, msg, input.column(), input.line_number())
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressInvalid => write!(f, "ast::address_invalid"),
            Self::AddressItemInvalid => write!(f, "ast::address_item_invalid"),
            Self::IdentInvalidStart => write!(f, "ast::ident_invalid_start"),
            Self::IdentInvalidChar => write!(f, "ast::ident_invalid_char"),
            Self::PunctInvalid => write!(f, "ast::punct_invalid"),
            Self::KeywordInvalid => write!(f, "ast::keyword_invalid"),
            Self::LiteralInvalid => write!(f, "ast::literal_invalid"),
            Self::ExpectedEither => write!(f, "ast::expected_either"),
            Self::UnexpectedEnd => write!(f, "ast::unexpected_end"),
            Self::OpCodeInvalid => write!(f, "ast::opcode_invalid"),
            Self::OperInvalid => write!(f, "ast::operator_invalid"),
            Self::RegisterInvalid => write!(f, "ast::register_invalid"),
            Self::DirectiveInvalid => write!(f, "ast::directive_invalid"),
            Self::RootInvalid => write!(f, "ast::root_invalid"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub msg: std::borrow::Cow<'static, str>,
    pub col: usize,
    pub line: usize,
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
impl std::error::Error for Error {}
