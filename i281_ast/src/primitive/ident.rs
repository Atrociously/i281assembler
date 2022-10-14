use i281_core::TokenIter;

use crate::{ErrorCode, ParseItem, Result};

#[derive(Clone, Debug)]
pub struct Ident(String);

impl Ident {
    const VALID_CHARS: &'static str =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";
    const VALID_START: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ParseItem for Ident {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let token = input.next().ok_or(ErrorCode::unexpected_end("ident", input))?;
        // safe because token has at least size 1
        let first = token.chars().next().unwrap();
        if !Self::VALID_START.contains(first) {
            // ident that does not start with valid char
            return Err(ErrorCode::IdentInvalidStart.into_err("ident must start with 'a-z'|'A-Z'|'_'", input));
        }
        if !token.chars().all(|c| Self::VALID_CHARS.contains(c)) {
            // ident contains invalid characters
            return Err(ErrorCode::IdentInvalidChar.into_err("ident must only contain 'a-z'|'A-Z'|'_'|'-'", input));
        }
        Ok(Ident(token))
    }
}
