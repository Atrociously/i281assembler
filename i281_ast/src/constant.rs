use super::{Ident, punct, Literal};

pub struct Constant(Ident, punct::Eq, Literal);
