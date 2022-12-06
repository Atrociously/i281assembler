#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

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

pub use address::{Address, AddressExpr, AddressItem};
pub use directive::Directive;
pub use instruction::Instruction;
pub use label::Label;
pub use pointer::Pointer;
pub use primitive::{keyword, literal, opcode, Ident, Literal, OpCode, Oper, Register};
pub use root::Root;
pub use variable::Variable;

pub use error::ParseError;
// The custom error span this will contain the input and extra will be the full input
pub type Span<'a> = nom_locate::LocatedSpan<&'a str, &'a str>;
pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

// this is an implementation trait used internally
pub(crate) trait ParseNom: Sized {
    // implementing this should generally follow these guidelines:
    //  - do not consume whitespace at the start or end unless it is meaningful
    //  - only consume exactly what is required for parsing nothing less nothing more
    fn parse(input: Span) -> IResult<Self>;
}

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
        <T as ParseNom>::parse(Span::new_extra(input, input))
    }
}
