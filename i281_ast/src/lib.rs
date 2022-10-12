mod primitive;

mod address;
mod directive;
mod error;
mod instruction;
mod label;
mod variable;
mod root;

mod util;

use i281_core::TokenIter;

pub use primitive::{
    keyword, literal, opcode, punct, Ident, Literal, OpCode, Oper, Punct, Register,
};

pub use address::{Address, AddressExpr, AddressItem};
pub use directive::Directive;
pub use error::Error;
pub use instruction::Instruction;
pub use label::Label;
pub use variable::Variable;
pub use root::Root;

pub use color_eyre::Result;

macro_rules! type_enum {
    ($name:ident {
        $($variant:ident $(($data:ty))?),*
        $(,)?
    }) => {
        $(
        #[derive(Clone, Debug)]
        pub struct $variant$((pub $data))?;
        )*

        paste::paste! {
            pub trait [<$name Trait>]: sealed::Sealed {}
        }

        mod sealed {
            pub trait Sealed {}
        }

        #[derive(Clone, Debug)]
        pub enum $name {
            $($variant($variant)),*
        }
    }
}
pub(crate) use type_enum;

mod sealed {
    use i281_core::TokenIter;
    use color_eyre::Result;

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
            |c| c.is_whitespace() || c.is_control()
        );
        <T as ParseItem>::parse(&mut input)
    }
}
