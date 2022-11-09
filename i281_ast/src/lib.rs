mod primitive;

mod address;
pub mod directive;
mod instruction;
mod label;
mod pointer;
mod root;
mod variable;

mod error;
mod util;

pub(crate) use util::type_enum;

pub use primitive::{keyword, literal, opcode, Ident, Literal, OpCode, Oper, Register};
pub use address::{Address, AddressExpr, AddressItem};
pub use directive::Directive;
pub use instruction::Instruction;
pub use label::Label;
pub use pointer::Pointer;
pub use root::Root;
pub use variable::Variable;

pub use error::ParseError;
pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

// this is an implementation trait used internally
mod sealed {
    pub trait ParseNom: Sized {
        fn parse(input: super::Span) -> super::IResult<Self>;
    }
}
pub(crate) use sealed::ParseNom;

// this is a public facing trait that is automatically implemented for all ParseNom implementors
pub trait Parse {
    fn parse(input: &str) -> IResult<Self>
    where
        Self: Sized;
}

impl<T> Parse for T
where
    T: ParseNom,
{
    fn parse(input: &str) -> IResult<Self> {
        <T as ParseNom>::parse(Span::new(input))
    }
}
