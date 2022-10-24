use i281_core::TokenIter;

use crate::{ErrorCode, punct, util, literal, Ident, Oper, ParseItem, Register, Result};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Address {
    pub to: AddressExpr,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AddressItem {
    Var(Ident),
    Reg(Register),
    Lit(literal::Byte),
}

impl AddressItem {
    /// Returns true if the address item can be evaluated at compile time
    pub fn is_const(&self) -> bool {
        matches!(self, Self::Var(..) | Self::Lit(..))
    }

    pub fn as_var(&self) -> Option<&Ident> {
        if let Self::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_reg(&self) -> Option<&Register> {
        if let Self::Reg(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_lit(&self) -> Option<&literal::Byte> {
        if let Self::Lit(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AddressExpr {
    Item(AddressItem),
    Expr {
        left: AddressItem,
        oper: Oper,
        right: Box<AddressExpr>,
    },
}

impl AddressExpr {
    pub fn is_const(&self) -> bool {
        self.iter().all(|(item, _)| item.is_const())
    }

    pub fn iter(&self) -> AddrIter {
        AddrIter {
            current: Some(self),
        }
    }
}

pub struct AddrIter<'a> {
    current: Option<&'a AddressExpr>,
}

impl<'a> Iterator for AddrIter<'a> {
    type Item = (&'a AddressItem, Option<&'a Oper>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            Some(AddressExpr::Item(item)) => {
                self.current = None;
                Some((item, None))
            },
            Some(AddressExpr::Expr { left, oper, right }) => {
                self.current = Some(right);
                Some((left, Some(oper)))
            },
            None => { None }
        }
    }
}

impl ParseItem for Address {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let to = util::parse_surround::<punct::OpenBracket, punct::CloseBracket, _, _, _>(
            input,
            |input: &mut TokenIter<I>| AddressExpr::parse(input),
        )?
        .pop()
        .ok_or(ErrorCode::AddressInvalid.into_err("no items in address brackets", input))?;

        Ok(Self { to })
    }
}

impl ParseItem for AddressItem {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let next = input.next().ok_or(ErrorCode::unexpected_end("address_item", input))?;
        let mut next = next.chars();
        // try to parse a literal
        match <literal::Byte as crate::Parse>::parse(&mut next.clone()) {
            Ok(lit) => Ok(Self::Lit(lit)),
            // literal failed try a register
            Err(..) => match <Register as crate::Parse>::parse(&mut next.clone()) {
                Ok(reg) => Ok(Self::Reg(reg)),
                // register failed try ident
                Err(..) => match <Ident as crate::Parse>::parse(&mut next) {
                    Ok(ident) => Ok(Self::Var(ident)),
                    // all possible options failed this is an invalid address item
                    Err(..) => Err(ErrorCode::AddressItemInvalid.into_err("expected either a literal, register, or variable identifier found none", input)),
                },
            },
        }
    }
}

impl ParseItem for AddressExpr {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let left = AddressItem::parse(input)?;
        let next = input.peek();

        match next {
            // we reached the closing bracket this is the final item in the expression
            Some(s) if s.len() == 1 && s.chars().next().unwrap() == punct::CloseBracket::REPR => {
                Ok(Self::Item(left))
            }
            Some(_) => {
                let oper = Oper::parse(input)?;
                let right = AddressExpr::parse(input)?;
                Ok(Self::Expr {
                    left,
                    oper,
                    right: Box::new(right),
                })
            }
            None => Err(ErrorCode::unexpected_end("address_expr", input)),
        }
    }
}
