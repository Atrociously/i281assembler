use either::Either;

use super::{opcode, Register, punct, Literal, Ident};

pub enum Instruction {
    NoOp(opcode::NoOp),
    InputC(opcode::InputC),
    InputCF(opcode::InputCF),
    InputD(opcode::InputD),
    InputDF(opcode::InputDF),
    Move(opcode::Move, Register, punct::Comma, Register),
    LoadI(opcode::LoadI, Register, punct::Comma, Literal),
    LoadP(opcode::LoadP),
    Add(opcode::Add, Register, punct::Comma, Register),
    AddI(opcode::AddI, Register, punct::Comma, Literal),
    Sub(opcode::Sub, Register, punct::Comma, Register),
    SubI(opcode::SubI, Register, punct::Comma, Literal),
    Load(opcode::Load),
    LoadF(opcode::LoadF),
    Store(opcode::Store),
    StoreF(opcode::StoreF),
    ShiftL(opcode::ShiftL, Register),
    ShiftR(opcode::ShiftR, Register),
    Cmp(opcode::Cmp, Register, punct::Comma, Register),
    Jump(opcode::Jump, Ident),
    BrE(Either<opcode::BrE, opcode::BrZ>, Ident),
    BrNE(Either<opcode::BrNE, opcode::BrNZ>, Ident),
    BrG(opcode::BrG, Ident),
    BrGE(opcode::BrGE, Ident),
}
