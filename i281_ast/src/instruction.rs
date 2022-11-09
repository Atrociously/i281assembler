use crate::util::{ws0, ws_end1};
use crate::{sealed::ParseNom, Span};

use nom::{bytes::complete::tag, sequence::separated_pair};

use crate::{literal, Address, IResult, Ident, OpCode, Pointer, Register};

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
    LoadP(Register, Pointer),
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
    pub(crate) fn parse_after_opcode(opcode: OpCode, input: Span) -> IResult<Self> {
        match opcode {
            OpCode::NoOp(..) => Ok((input, Self::NoOp)),
            OpCode::InputC(..) => {
                let (input, addr) = Address::parse(input)?;
                Ok((input, Self::InputC(addr)))
            }
            OpCode::InputCF(..) => {
                let (input, addr) = Address::parse(input)?;
                Ok((input, Self::InputCF(addr)))
            }
            OpCode::InputD(..) => {
                let (input, addr) = Address::parse(input)?;
                Ok((input, Self::InputD(addr)))
            }
            OpCode::InputDF(..) => {
                let (input, addr) = Address::parse(input)?;
                Ok((input, Self::InputDF(addr)))
            }
            OpCode::Move(..) => {
                let (input, (rx, ry)) =
                    separated_pair(Register::parse, ws0(tag(",")), Register::parse)(input)?;
                Ok((input, Self::Move(rx, ry)))
            }
            OpCode::LoadI(..) => {
                let (input, (rx, value)) =
                    separated_pair(Register::parse, ws0(tag(",")), literal::Byte::parse)(input)?;
                Ok((input, Self::LoadI(rx, value)))
            }
            OpCode::LoadP(..) => {
                let (input, (rx, value)) =
                    separated_pair(Register::parse, ws0(tag(",")), Pointer::parse)(input)?;
                Ok((input, Self::LoadP(rx, value)))
            }
            OpCode::Add(..) => {
                let (input, (rx, ry)) =
                    separated_pair(Register::parse, ws0(tag(",")), Register::parse)(input)?;
                Ok((input, Self::Add(rx, ry)))
            }
            OpCode::AddI(..) => {
                let (input, (rx, value)) =
                    separated_pair(Register::parse, ws0(tag(",")), literal::Byte::parse)(input)?;
                Ok((input, Self::AddI(rx, value)))
            }
            OpCode::Sub(..) => {
                let (input, (rx, ry)) =
                    separated_pair(Register::parse, ws0(tag(",")), Register::parse)(input)?;
                Ok((input, Self::Sub(rx, ry)))
            }
            OpCode::SubI(..) => {
                let (input, (rx, value)) =
                    separated_pair(Register::parse, ws0(tag(",")), literal::Byte::parse)(input)?;
                Ok((input, Self::SubI(rx, value)))
            }
            OpCode::Load(..) => {
                let (input, (rx, addr)) =
                    separated_pair(Register::parse, ws0(tag(",")), Address::parse)(input)?;
                Ok((input, Self::Load(rx, addr)))
            }
            OpCode::LoadF(..) => {
                let (input, (rx, addr)) =
                    separated_pair(Register::parse, ws0(tag(",")), Address::parse)(input)?;
                Ok((input, Self::LoadF(rx, addr)))
            }
            OpCode::Store(..) => {
                let (input, (addr, rx)) =
                    separated_pair(Address::parse, ws0(tag(",")), Register::parse)(input)?;
                Ok((input, Self::Store(addr, rx)))
            }
            OpCode::StoreF(..) => {
                let (input, (addr, rx)) =
                    separated_pair(Address::parse, ws0(tag(",")), Register::parse)(input)?;
                Ok((input, Self::StoreF(addr, rx)))
            }
            OpCode::ShiftL(..) => {
                let (input, rx) = Register::parse(input)?;
                Ok((input, Self::ShiftL(rx)))
            }
            OpCode::ShiftR(..) => {
                let (input, rx) = Register::parse(input)?;
                Ok((input, Self::ShiftR(rx)))
            }
            OpCode::Cmp(..) => {
                let (input, (rx, ry)) =
                    separated_pair(Register::parse, ws0(tag(",")), Register::parse)(input)?;
                Ok((input, Self::Cmp(rx, ry)))
            }
            OpCode::Jump(..) => {
                let (input, label) = Ident::parse(input)?;
                Ok((input, Self::Jump(label)))
            }
            OpCode::BrE(..) | OpCode::BrZ(..) => {
                let (input, label) = Ident::parse(input)?;
                Ok((input, Self::BrE(label)))
            }
            OpCode::BrNE(..) | OpCode::BrNZ(..) => {
                let (input, label) = Ident::parse(input)?;
                Ok((input, Self::BrNE(label)))
            }
            OpCode::BrG(..) => {
                let (input, label) = Ident::parse(input)?;
                Ok((input, Self::BrG(label)))
            }
            OpCode::BrGE(..) => {
                let (input, label) = Ident::parse(input)?;
                Ok((input, Self::BrGE(label)))
            }
        }
    }
}

impl ParseNom for Instruction {
    fn parse(input: Span) -> IResult<Self> {
        let (input, opcode) = ws_end1(OpCode::parse)(input)?;
        Self::parse_after_opcode(opcode, input)
    }
}
