use color_eyre::Result;

use i281_core::TokenIter;

use crate::{error::Error, punct, util, Ident, Literal, Oper, ParseItem, Register};

#[derive(Clone, Debug)]
pub struct Address {
    pub to: AddressExpr,
}

#[derive(Clone, Debug)]
pub enum AddressItem {
    Var(Ident),
    Reg(Register),
    Lit(Literal),
}

#[derive(Clone, Debug)]
pub enum AddressExpr {
    Item(AddressItem),
    Expr {
        left: Box<AddressExpr>,
        oper: Oper,
        right: Box<AddressExpr>,
    },
}

impl ParseItem for Address {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let to = util::parse_surround::<punct::OpenBracket, punct::CloseBracket, _, _, _>(
            input,
            |input: &mut TokenIter<I>| AddressExpr::parse(input),
        )?
        .pop()
        .ok_or(Error::InvalidAddressExpr)?;

        Ok(Self { to })
    }
}

impl ParseItem for AddressItem {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let next = input.next().ok_or(Error::InvalidAddressItem)?;
        let mut next = next.chars();
        // try to parse a literal
        match <Literal as crate::Parse>::parse(&mut next.clone()) {
            Ok(lit) => Ok(Self::Lit(lit)),
            // literal failed try a register
            Err(..) => match <Register as crate::Parse>::parse(&mut next.clone()) {
                Ok(reg) => Ok(Self::Reg(reg)),
                // register failed try ident
                Err(..) => match <Ident as crate::Parse>::parse(&mut next) {
                    Ok(ident) => Ok(Self::Var(ident)),
                    // all possible options failed this is an invalid address item
                    Err(..) => Err(Error::InvalidAddressItem.into()),
                },
            },
        }
    }
}

impl ParseItem for AddressExpr {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let left = AddressItem::parse(input).map(Self::Item)?;
        let next = input.peek();

        match next {
            // we reached the closing bracket this is the final item in the expression
            Some(s) if s.len() == 1 && s.chars().next().unwrap() == punct::CloseBracket::REPR => {
                Ok(left)
            }
            Some(_) => {
                let oper = Oper::parse(input)?;
                let right = AddressExpr::parse(input)?;
                Ok(Self::Expr {
                    left: Box::new(left),
                    oper,
                    right: Box::new(right),
                })
            }
            None => Err(Error::InvalidAddressExpr.into()),
        }
    }
}
