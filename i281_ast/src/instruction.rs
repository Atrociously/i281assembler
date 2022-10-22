use i281_core::TokenIter;

use crate::{
    literal, punct::Comma, util::parse_sep, Address, Ident, OpCode, ParseItem, Register, Result,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Instruction {
    NoOp,
    InputC(Address),
    InputCF(Address),
    InputD(Address),
    InputDF(Address),
    Move(Register, Register),
    LoadI(Register, literal::Byte),
    LoadP(Register, literal::Byte),
    Add(Register, Register),
    AddI(Register, literal::Byte),
    Sub(Register, Register),
    SubI(Register, literal::Byte),
    Load(Register, Address),
    LoadF(Register, Address),
    Store(Address, Register),
    StoreF(Address, Register),
    ShiftL(Register),
    ShiftR(Register),
    Cmp(Register, Register),
    Jump(Ident),
    BrE(Ident),
    BrNE(Ident),
    BrG(Ident),
    BrGE(Ident),
}

impl Instruction {
    pub(crate) fn parse_after_opcode<I>(opcode: OpCode, input: &mut TokenIter<I>) -> Result<Self>
    where
        I: Iterator<Item = char>,
    {
        match opcode {
            OpCode::NoOp(..) => Ok(Self::NoOp),
            OpCode::InputC(..) => {
                let addr = Address::parse(input)?;
                Ok(Self::InputC(addr))
            }
            OpCode::InputCF(..) => {
                let addr = Address::parse(input)?;
                Ok(Self::InputCF(addr))
            }
            OpCode::InputD(..) => {
                let addr = Address::parse(input)?;
                Ok(Self::InputD(addr))
            }
            OpCode::InputDF(..) => {
                let addr = Address::parse(input)?;
                Ok(Self::InputDF(addr))
            }
            OpCode::Move(..) => {
                let (rx, ry) = parse_sep::<Register, Comma, Register, I>(input)?;
                Ok(Self::Move(rx, ry))
            }
            OpCode::LoadI(..) => {
                let (rx, value) = parse_sep::<Register, Comma, literal::Byte, I>(input)?;
                Ok(Self::LoadI(rx, value))
            }
            OpCode::LoadP(..) => {
                let (rx, value) = parse_sep::<Register, Comma, literal::Byte, I>(input)?;
                Ok(Self::LoadP(rx, value))
            }
            OpCode::Add(..) => {
                let (rx, ry) = parse_sep::<Register, Comma, Register, I>(input)?;
                Ok(Self::Add(rx, ry))
            }
            OpCode::AddI(..) => {
                let (rx, value) = parse_sep::<Register, Comma, literal::Byte, I>(input)?;
                Ok(Self::AddI(rx, value))
            }
            OpCode::Sub(..) => {
                let (rx, ry) = parse_sep::<Register, Comma, Register, I>(input)?;
                Ok(Self::Sub(rx, ry))
            }
            OpCode::SubI(..) => {
                let (rx, value) = parse_sep::<Register, Comma, literal::Byte, I>(input)?;
                Ok(Self::SubI(rx, value))
            }
            OpCode::Load(..) => {
                let (rx, addr) = parse_sep::<Register, Comma, Address, I>(input)?;
                Ok(Self::Load(rx, addr))
            }
            OpCode::LoadF(..) => {
                let (rx, addr) = parse_sep::<Register, Comma, Address, I>(input)?;
                Ok(Self::LoadF(rx, addr))
            }
            OpCode::Store(..) => {
                let (addr, rx) = parse_sep::<Address, Comma, Register, I>(input)?;
                Ok(Self::Store(addr, rx))
            }
            OpCode::StoreF(..) => {
                let (addr, rx) = parse_sep::<Address, Comma, Register, I>(input)?;
                Ok(Self::StoreF(addr, rx))
            }
            OpCode::ShiftL(..) => {
                let rx = Register::parse(input)?;
                Ok(Self::ShiftL(rx))
            }
            OpCode::ShiftR(..) => {
                let rx = Register::parse(input)?;
                Ok(Self::ShiftR(rx))
            }
            OpCode::Cmp(..) => {
                let (rx, ry) = parse_sep::<Register, Comma, Register, I>(input)?;
                Ok(Self::Cmp(rx, ry))
            }
            OpCode::Jump(..) => {
                let label = Ident::parse(input)?;
                Ok(Self::Jump(label))
            }
            OpCode::BrE(..) => {
                let label = Ident::parse(input)?;
                Ok(Self::BrE(label))
            }
            OpCode::BrNE(..) => {
                let label = Ident::parse(input)?;
                Ok(Self::BrNE(label))
            }
            OpCode::BrG(..) => {
                let label = Ident::parse(input)?;
                Ok(Self::BrG(label))
            }
            OpCode::BrGE(..) => {
                let label = Ident::parse(input)?;
                Ok(Self::BrGE(label))
            }
        }
    }
}

impl ParseItem for Instruction {
    fn parse<I: Iterator<Item = char>>(input: &mut TokenIter<I>) -> Result<Self> {
        let opcode = OpCode::parse(input)?;
        Self::parse_after_opcode(opcode, input)
    }
}
