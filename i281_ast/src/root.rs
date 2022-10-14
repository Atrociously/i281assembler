use i281_core::TokenIter;

use crate::{Directive, ParseItem, Result, directive, ErrorCode};

#[derive(Clone, Debug)]
pub struct Root {
    pub data: Option<directive::Data>,
    pub code: directive::Code,
}

impl ParseItem for Root {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let mut data: Option<directive::Data> = None;
        let mut code: Option<directive::Code> = None;

        while let Some(_) = input.peek() {
            let directive = Directive::parse(input)?;
            match directive {
                Directive::Data(d) => match data {
                    Some(_) => return Err(ErrorCode::RootInvalid.into_err("multiple data directives defined", input)),
                    None => data = Some(d),
                },
                Directive::Code(c) => match code {
                    Some(_) => return Err(ErrorCode::RootInvalid.into_err("multiple code directives defined", input)),
                    None => code = Some(c),
                }
            }
        }
        Ok(Self {
            data,
            code: code.ok_or(ErrorCode::RootInvalid.into_err("no code directive defined", input))?,
        })
    }
}
