#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BranchCondition {
    #[cfg_attr(feature = "serde", serde(rename = "eq"))]
    Equal = 0b00,
    #[cfg_attr(feature = "serde", serde(rename = "ne"))]
    NotEqual = 0b01,
    #[cfg_attr(feature = "serde", serde(rename = "gr"))]
    Greater = 0b10,
    #[cfg_attr(feature = "serde", serde(rename = "ge"))]
    GreaterEqual = 0b11,
}

impl From<BranchCondition> for u16 {
    fn from(v: BranchCondition) -> Self {
        Self::from(v as u8)
    }
}
