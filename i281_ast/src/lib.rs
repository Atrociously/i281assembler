mod primitive;

mod constant;
mod directive;
mod label;
mod instruction;

pub use primitive::{Ident, OpCode, Literal, Oper, Punct, Register, Token, opcode, punct};

macro_rules! type_enum {
    ($name:ident {
        $($variant:ident $(: $type:ty = $repr:literal)?),*
        $(,)?
    }) => {
        $(
        #[derive(Clone, Copy, Debug)]
        pub struct $variant;

        $(impl $variant {
            const REPR: $type = $repr;
        })?
        )*

        paste::paste! {
            pub trait [<$name Trait>]: sealed::Sealed {}
        }

        mod sealed {
            pub trait Sealed {}
        }

        pub enum $name {
            $($variant($variant)),*
        }
    }
}
pub(crate) use type_enum;
