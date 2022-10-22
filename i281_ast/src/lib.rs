mod primitive;

mod address;
pub mod directive;
mod error;
mod instruction;
mod label;
mod root;
mod variable;

mod util;

use i281_core::TokenIter;

pub use primitive::{
    keyword, literal, opcode, punct, Ident, Literal, OpCode, Oper, Punct, Register,
};

pub use address::{Address, AddressExpr, AddressItem};
pub use directive::Directive;
pub use error::{Error, ErrorCode};
pub use instruction::Instruction;
pub use label::Label;
pub use root::Root;
pub use variable::Variable;

//pub use color_eyre::Result;
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! type_enum {
    (@base $name:ident {$($variant:ident),*}) => {
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub enum $name {
            $($variant($variant)),*
        }
    };
    ($name:ident {
        $($variant:ident $(($data:ty))?),*
        $(,)?
    }) => {
        $(
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub struct $variant$((pub $data))?;
        )*

        type_enum!(@base $name {$($variant),*});
    };
}
pub(crate) use type_enum;

mod sealed {
    use super::Result;
    use i281_core::TokenIter;

    pub trait ParseItem: Sized {
        fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self>;
    }
}
pub(crate) use sealed::ParseItem;

// this trait must only ever operate on one line of characters at a time some parse implementations
// rely on this property to correctly parse
pub trait Parse: ParseItem {
    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self>;
}

impl<T: ParseItem> Parse for T {
    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self> {
        let mut input = TokenIter::new_with(
            input,
            |c| Punct::is_punct(c),
            punct::SemiColon::REPR,
            |c| c.is_whitespace() || c.is_control(),
        );
        <T as ParseItem>::parse(&mut input)
    }
}
