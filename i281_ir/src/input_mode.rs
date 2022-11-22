#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum InputMode {
    C = 0b00,
    CF = 0b01,
    D = 0b10,
    DF = 0b11,
}

impl From<InputMode> for u16 {
    fn from(v: InputMode) -> Self {
        Self::from(v as u8)
    }
}
