use nom::{branch::alt, combinator::map};

use crate::{type_enum, ParseNom};

macro_rules! opcode {
    ($($variant:ident == $val:literal),+ $(,)?) => {
        type_enum!(OpCode {
            $($variant),+
        });

        $(
        impl $crate::ParseNom for $variant {
            fn parse(input: crate::Span) -> crate::IResult<Self> {
                nom::bytes::complete::tag_no_case($val)(input)
                    .map(|(input, _)| (input, Self))
            }
        }
        )+

        impl OpCode {
            pub const ALL: &'static [&'static str] = &[$($val),+];
        }
    }
}

opcode! {
    NoOp == "NOOP",
    InputC == "INPUTC",
    InputCF == "INPUTCF",
    InputD == "INPUTD",
    InputDF == "INPUTDF",

    Move == "MOVE",
    LoadI == "LOADI",
    LoadP == "LOADP",

    Add == "ADD",
    AddI == "ADDI",
    Sub == "SUB",
    SubI == "SUBI",

    Load == "LOAD",
    LoadF == "LOADF",
    Store == "STORE",
    StoreF == "STOREF",

    ShiftL == "SHIFTL",
    ShiftR == "SHIFTR",
    Cmp == "CMP",
    Jump == "JUMP",
    BrE == "BRE",
    BrZ == "BRZ",
    BrNE == "BRNE",
    BrNZ == "BRNZ",
    BrG == "BRG",
    BrGE == "BRGE",
}

impl ParseNom for OpCode {
    fn parse(input: crate::Span) -> crate::IResult<Self> {
        alt((
            map(NoOp::parse, Self::NoOp),
            map(InputCF::parse, Self::InputCF),
            map(InputC::parse, Self::InputC),
            map(InputDF::parse, Self::InputDF),
            map(InputD::parse, Self::InputD),
            map(Move::parse, Self::Move),
            map(LoadI::parse, Self::LoadI),
            map(LoadP::parse, Self::LoadP),
            map(AddI::parse, Self::AddI),
            map(Add::parse, Self::Add),
            map(SubI::parse, Self::SubI),
            map(Sub::parse, Self::Sub),
            map(LoadF::parse, Self::LoadF),
            map(Load::parse, Self::Load),
            map(StoreF::parse, Self::StoreF),
            map(Store::parse, Self::Store),
            map(ShiftL::parse, Self::ShiftL),
            map(ShiftR::parse, Self::ShiftR),
            map(Cmp::parse, Self::Cmp),
            map(Jump::parse, Self::Jump),
            alt((
                map(BrE::parse, Self::BrE),
                map(BrZ::parse, Self::BrZ),
                map(BrNE::parse, Self::BrNE),
                map(BrNZ::parse, Self::BrNZ),
                map(BrGE::parse, Self::BrGE),
                map(BrG::parse, Self::BrG),
            )),
        ))(input)
    }
}
