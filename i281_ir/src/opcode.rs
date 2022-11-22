#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum OpCode {
    NoOp = 0b0000,
    Input = 0b0001,
    Move = 0b0010,
    LoadI = 0b0011,
    Add = 0b0100,
    AddI = 0b0101,
    Sub = 0b0110,
    SubI = 0b0111,
    Load = 0b1000,
    LoadF = 0b1001,
    Store = 0b1010,
    StoreF = 0b1011,
    Shift = 0b1100,
    Cmp = 0b1101,
    Jump = 0b1110,
    Branch = 0b1111,
}

impl From<i281_ast::OpCode> for OpCode {
    fn from(code: i281_ast::OpCode) -> Self {
        match code {
            i281_ast::OpCode::NoOp(_) => Self::NoOp,
            i281_ast::OpCode::InputC(_)
            | i281_ast::OpCode::InputCF(_)
            | i281_ast::OpCode::InputD(_)
            | i281_ast::OpCode::InputDF(_) => Self::Input,
            i281_ast::OpCode::Move(_) => Self::Move,
            i281_ast::OpCode::LoadI(_) | i281_ast::OpCode::LoadP(_) => Self::LoadI,
            i281_ast::OpCode::Add(_) => Self::Add,
            i281_ast::OpCode::AddI(_) => Self::AddI,
            i281_ast::OpCode::Sub(_) => Self::Sub,
            i281_ast::OpCode::SubI(_) => Self::SubI,
            i281_ast::OpCode::Load(_) => Self::Load,
            i281_ast::OpCode::LoadF(_) => Self::LoadF,
            i281_ast::OpCode::Store(_) => Self::Store,
            i281_ast::OpCode::StoreF(_) => Self::StoreF,
            i281_ast::OpCode::ShiftL(_) | i281_ast::OpCode::ShiftR(_) => Self::Shift,
            i281_ast::OpCode::Cmp(_) => Self::Cmp,
            i281_ast::OpCode::Jump(_) => Self::Jump,
            i281_ast::OpCode::BrE(_)
            | i281_ast::OpCode::BrZ(_)
            | i281_ast::OpCode::BrNE(_)
            | i281_ast::OpCode::BrNZ(_)
            | i281_ast::OpCode::BrG(_)
            | i281_ast::OpCode::BrGE(_) => Self::Branch,
        }
    }
}

impl From<OpCode> for u16 {
    fn from(v: OpCode) -> Self {
        Self::from(v as u8)
    }
}
