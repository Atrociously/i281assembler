#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Register {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    D = 0b11,
}

impl From<i281_ast::Register> for Register {
    fn from(reg: i281_ast::Register) -> Self {
        match reg {
            i281_ast::Register::A => Self::A,
            i281_ast::Register::B => Self::B,
            i281_ast::Register::C => Self::C,
            i281_ast::Register::D => Self::D,
        }
    }
}

impl From<&i281_ast::Register> for Register {
    fn from(reg: &i281_ast::Register) -> Self {
        Self::from(*reg)
    }
}

impl From<Register> for u16 {
    fn from(v: Register) -> Self {
        Self::from(v as u8)
    }
}
