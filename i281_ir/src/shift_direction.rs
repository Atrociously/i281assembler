#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ShiftDirection {
    Left = 0b0,
    Right = 0b1,
}

impl From<ShiftDirection> for u16 {
    fn from(v: ShiftDirection) -> Self {
        Self::from(v as u8)
    }
}
