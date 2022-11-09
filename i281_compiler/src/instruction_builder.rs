#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionBuilder(u16);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOpcode {
    NOOP   = 0b0000,
    INPUT  = 0b0001,
    MOVE   = 0b0010,
    LOADI  = 0b0011,
    ADD    = 0b0100,
    ADDI   = 0b0101,
    SUB    = 0b0110,
    SUBI   = 0b0111,
    LOAD   = 0b1000,
    LOADF  = 0b1001,
    STORE  = 0b1010,
    STOREF = 0b1011,
    SHIFT  = 0b1100,
    CMP    = 0b1101,
    JUMP   = 0b1110,
    BRANCH = 0b1111,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinRegister {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    D = 0b11,
}

macro_rules! into_num {
    ($name:ident = $($nty:ty),*) => {
        $(
        impl From<$name> for $nty {
            fn from(v: $name) -> Self {
                Self::from(v as u8)
            }
        }
        )*
    };
}

into_num!(BinOpcode = u16, u32, u64);
into_num!(BinRegister = u16, u32, u64);

impl InstructionBuilder {
    pub fn new() -> Self {
        Self(0b0000000000000000)
    }

    pub fn set_opcode(&mut self, opcode: BinOpcode) {
        let opcode = u16::from(opcode) << 12;
        let zero: u16 = 0b0000_11_11_11111111;

        // zero out the existing opcode then set to new opcode
        self.0 = (self.0 & zero) | opcode;
    }

    pub fn set_rx(&mut self, reg: BinRegister) {
        let reg = u16::from(reg) << 10;
        let zero: u16 = 0b1111_00_11_11111111;

        self.0 = (self.0 & zero) | reg;
    }

    pub fn set_ry(&mut self, reg: BinRegister) {
        let reg = u16::from(reg) << 8;
        let zero: u16 = 0b1111_11_00_11111111;

        self.0 = (self.0 & zero) | reg;
    }

    pub fn set_data(&mut self, data: u8) {
        let zero: u16 = 0b1111_11_11_00000000;

        self.0 = (self.0 & zero) | u16::from(data);
    }
}

impl std::fmt::Binary for InstructionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.0, f)
    }
}
