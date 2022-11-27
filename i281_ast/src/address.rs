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

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Address {
    pub to: AddressExpr,
}

impl AsRef<Address> for Address {
    fn as_ref(&self) -> &Address {
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

    /// Get an iterator over all variables within the address
    ///
    /// The parser will parse any valid address syntax so there may be more than one variable
    pub fn variables(&self) -> impl Iterator<Item = &Ident> {
        self.iter().filter_map(|(item, _)| item.as_var())
    }

    /// Get an iterator over all registers in the address
    ///
    /// The parser will parse any number of registers however usually only one is allowed
    pub fn registers<'a>(&'a self) -> impl Iterator<Item = Register> + 'a {
        self.iter().filter_map(|(item, _)| item.as_reg().copied())
    }

    /// Get an iterator over the address expression
    ///
    /// May only iterate once
    pub fn iter(&self) -> AddrIter {
        AddrIter {
            current: Some(self),
        }
    }
}

/// An iterator over an address expression
///
/// This will descend through the address's self references until it reaches the end.
/// The oper will always be Some until the last entry in the expression where it will be none.
/// The oper is the operator that comes after the current entry
/// Ex: [10+20] will iterate like so: (10, Some(+)), (20, None)
/// Ex2: [var+10-20] becomes: ("var", Some(+)), (10, Some(-)), (20, None)
#[allow(missing_debug_implementations)] // you shouldn't debug print this
#[derive(Clone, Copy)]
pub struct AddrIter<'a> {
    current: Option<&'a AddressExpr>,
}

impl<'a> Iterator for AddrIter<'a> {
    type Item = (&'a AddressItem, Option<&'a Oper>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.as_ref() {
            Some(AddressExpr::Item(item)) => {
                self.current = None; // we have reached the bottom of the expression
                Some((item, None)) // return ourself
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

impl std::fmt::Display for AddressItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(v) => v.fmt(f),
            Self::Reg(r) => r.fmt(f),
            Self::Lit(l) => l.fmt(f),
        }
    }
}

impl std::fmt::Display for AddressExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self
            .iter()
            .map(|(item, oper)| {
                let item = item.to_string();
                let oper = oper.map(|v| v.to_string()).unwrap_or_else(|| "".to_owned());
                item + &oper
            })
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{v}")
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.to)
    }
}

#[cfg(test)]
mod test {
    use super::{Address, AddressExpr, AddressItem};
    use crate::{literal::Byte, Oper, Parse, Register};

    #[test]
    fn address1() {
        let expected = Address {
            to: AddressExpr::Expr {
                left: AddressItem::Lit(Byte(10).into()),
                oper: Oper::Add,
                right: Box::new(AddressExpr::Expr {
                    left: AddressItem::Reg(Register::A),
                    oper: Oper::Sub,
                    right: Box::new(AddressExpr::Item(AddressItem::Var("var".into()))),
                }),
            },
        };
        assert_eq!(Address::parse("[10+A-var]").unwrap().1, expected);
    }

    #[test]
    fn address2() {
        let expected = Address {
            to: AddressExpr::Expr {
                left: AddressItem::Lit(Byte(10).into()),
                oper: Oper::Add,
                right: Box::new(AddressExpr::Expr {
                    left: AddressItem::Var("var1".into()),
                    oper: Oper::Sub,
                    right: Box::new(AddressExpr::Item(AddressItem::Var("var2".into()))),
                }),
            },
        };
        assert_eq!(Address::parse("[10+var1-var2]").unwrap().1, expected);
    }

    #[test]
    fn address3() {
        let expected = Address {
            to: AddressExpr::Expr {
                left: AddressItem::Var("var".into()),
                oper: Oper::Add,
                right: Box::new(AddressExpr::Expr {
                    left: AddressItem::Var("var".into()),
                    oper: Oper::Sub,
                    right: Box::new(AddressExpr::Item(AddressItem::Reg(Register::C))),
                }),
            },
        };
        assert_eq!(Address::parse("[var + var - C]").unwrap().1, expected);
    }
}
