use crate::{error::Error, Parse};

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

impl Parse for Ident {
    type Err = Error;

    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err> {
        let first = input.next().ok_or(Error::InvalidIdent)?;
        if !Self::VALID_START.contains(first) {
            return Err(Error::InvalidIdent); // check the first ident char
        }
        let mut after = input.take_while(|c| !c.is_whitespace());
        if after.any(|c| !Self::VALID_CHARS.contains(c)) {
            return Err(Error::InvalidIdent); // check all the rest of the chars in the ident
        }
        let mut ident = first.to_string();
        ident.extend(after);
        Ok(Ident(ident))
    }
}
