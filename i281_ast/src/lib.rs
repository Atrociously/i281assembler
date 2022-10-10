mod primitive;

mod address;
mod directive;
mod error;
mod instruction;
mod label;
mod variable;

pub use primitive::{opcode, punct, Ident, Literal, OpCode, Oper, Punct, Register};

pub use address::{Address, AddressExpr, AddressItem};
pub use directive::Directive;
pub use instruction::Instruction;
pub use label::Label;
pub use variable::Variable;

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

        pub enum $name {
            $($variant($variant)),*
        }
    }
}
pub(crate) use type_enum;

// this trait must only ever operate on one line of characters at a time some parse implementations
// rely on this property to correctly parse
pub trait Parse: Sized {
    type Err;
    fn parse<I: Iterator<Item = char>>(input: &mut I) -> Result<Self, Self::Err>;
}

pub enum Token {
    Ident(Ident),
    OpCode(OpCode),
    Literal(Literal),
    Oper(Oper),
    Punct(Punct),
    Regiter(Register),
    Directive(Directive),
    Variable(Variable),
    Label(Label),
    Instruction(Instruction),
    Address(Address),
}
