use crate::util::{always_fails, ws0, ws_end1};
use crate::{opcode, ParseNom, Span};

use nom::{bytes::complete::tag, sequence::separated_pair};

use crate::{literal, Address, IResult, Ident, OpCode, Pointer, Register};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "opcode", content = "args", rename_all = "UPPERCASE")
)]
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
        let (input, ins) = always_fails(move |input| Self::parse_after_opcode(opcode.clone(), input))(input)?;
        return Ok((input, ins));
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoOp => write!(f, "{}", opcode::NoOp),
            Instruction::InputC(addr) => write!(f, "{} {addr}", opcode::InputC),
            Instruction::InputCF(addr) => write!(f, "{} {addr}", opcode::InputCF),
            Instruction::InputD(addr) => write!(f, "{} {addr}", opcode::InputD),
            Instruction::InputDF(addr) => write!(f, "{} {addr}", opcode::InputDF),
            Instruction::Move(rx, ry) => write!(f, "{} {rx}, {ry}", opcode::Move),
            Instruction::LoadI(rx, val) => write!(f, "{} {rx}, {val}", opcode::LoadI),
            Instruction::LoadP(rx, pointer) => write!(f, "{} {rx}, {pointer}", opcode::LoadP),
            Instruction::Add(rx, ry) => write!(f, "{} {rx}, {ry}", opcode::Add),
            Instruction::AddI(rx, val) => write!(f, "{} {rx}, {val}", opcode::AddI),
            Instruction::Sub(rx, ry) => write!(f, "{} {rx}, {ry}", opcode::Sub),
            Instruction::SubI(rx, val) => write!(f, "{} {rx}, {val}", opcode::SubI),
            Instruction::Load(rx, addr) => write!(f, "{} {rx}, {addr}", opcode::Load),
            Instruction::LoadF(rx, addr) => write!(f, "{} {rx}, {addr}", opcode::LoadF),
            Instruction::Store(addr, rx) => write!(f, "{} {addr}, {rx}", opcode::Store),
            Instruction::StoreF(addr, rx) => write!(f, "{} {addr}, {rx}", opcode::StoreF),
            Instruction::ShiftL(rx) => write!(f, "{} {rx}", opcode::ShiftL),
            Instruction::ShiftR(rx) => write!(f, "{} {rx}", opcode::ShiftR),
            Instruction::Cmp(rx, ry) => write!(f, "{} {rx}, {ry}", opcode::Cmp),
            Instruction::Jump(label) => write!(f, "{} {label}", opcode::Jump),
            Instruction::BrE(label) => write!(f, "{} {label}", opcode::BrE),
            Instruction::BrNE(label) => write!(f, "{} {label}", opcode::BrNE),
            Instruction::BrG(label) => write!(f, "{} {label}", opcode::BrG),
            Instruction::BrGE(label) => write!(f, "{} {label}", opcode::BrGE),
        }
    }
}

// TODO implement tests for all instructions
