#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    D = 0b11
}
