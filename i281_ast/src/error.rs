#[derive(Debug)]
pub enum Error {
    InvalidIdent,
    InvalidLiteral,
    InvalidDirective,
    InvalidOpCode,
    InvalidOper,
    InvalidPunct,
    InvalidRegister,
    InvalidKeyword,
    InvalidInstruction,
    InvalidAddressItem,
    InvalidAddressExpr,
    InvalidSurround,
    ExpectedOneOf,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl std::error::Error for Error {}
