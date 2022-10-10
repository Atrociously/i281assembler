use crate::{Ident, Literal, Oper, Register};

pub struct Address {
    pub to: AddressExpr,
}

pub enum AddressItem {
    Var(Ident),
    Reg(Register),
    Lit(Literal),
}

pub enum AddressExpr {
    Item(AddressItem),
    Expr {
        left: Box<AddressExpr>,
        oper: Oper,
        right: Box<AddressExpr>,
    },
}
