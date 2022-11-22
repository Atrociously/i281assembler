#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BranchCondition {
    Equal = 0b00,
    NotEqual = 0b01,
    Greater = 0b10,
    GreaterEqual = 0b11,
}

impl From<BranchCondition> for u16 {
    fn from(v: BranchCondition) -> Self {
        Self::from(v as u8)
    }
}
