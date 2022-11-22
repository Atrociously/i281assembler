use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, peek},
    sequence::delimited,
};

use crate::{
    literal::Byte,
    util::{ws0, ws_end0, ws_start0},
    Ident, Oper, ParseError, ParseNom, Register,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Address {
    pub to: AddressExpr,
}

impl AsRef<Address> for Address {
    fn as_ref(&self) -> &Address {
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AddressItem {
    Var(Ident),
    Reg(Register),
    Lit(Byte),
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

    pub fn as_lit(&self) -> Option<&Byte> {
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

#[allow(missing_debug_implementations)]
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
            }
            Some(AddressExpr::Expr { left, oper, right }) => {
                self.current = Some(right);
                Some((left, Some(oper)))
            }
            None => None,
        }
    }
}

impl ParseNom for Address {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, to) =
            delimited(ws_end0(tag("[")), AddressExpr::parse, ws_start0(tag("]")))(input)?;
        Ok((input, Self { to }))
    }
}

impl ParseNom for AddressItem {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        alt((
            map(Byte::parse, Self::Lit),
            map(Register::parse, Self::Reg),
            map(Ident::parse, Self::Var),
        ))(input)
    }
}

impl ParseNom for AddressExpr {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        let (input, left) = AddressItem::parse(input)?;

        if let Ok((input, _)) = peek(ws_start0(tag::<_, _, ParseError>("]")))(input) {
            Ok((input, Self::Item(left)))
        } else {
            let (input, oper) = ws0(Oper::parse)(input)?;
            let (input, right) = Self::parse(input)?;
            Ok((
                input,
                Self::Expr {
                    left,
                    oper,
                    right: Box::new(right),
                },
            ))
        }
    }
}
