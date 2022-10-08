mod ident;
mod literal;
pub mod opcode;
mod oper;
pub mod punct;
mod register;

pub use ident::Ident;
pub use opcode::OpCode;
pub use literal::Literal;
pub use oper::Oper;
pub use punct::Punct;
pub use register::Register;

pub enum Token {
    Ident(Ident),
    OpCode(OpCode),
    Literal(Literal),
    Oper(Oper),
    Punct(Punct),
    Regiter(Register),
}
