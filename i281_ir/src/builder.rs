use crate::{BranchCondition, InputMode, OpCode, Register, ShiftDirection};

pub struct InstructionBuilder(u16);

impl InstructionBuilder {
    pub fn new() -> Self {
        Self(0b0000_00_00_00000000)
    }

    pub fn set_opcode(&mut self, opcode: OpCode) -> &mut Self {
        self.set_a(u16::from(opcode));
        self
    }

    pub fn set_rx(&mut self, rx: Register) -> &mut Self {
        self.set_b(u16::from(rx));
        self
    }

    pub fn set_ry(&mut self, ry: Register) -> &mut Self {
        self.set_c(u16::from(ry));
        self
    }

    pub fn set_branch(&mut self, branch: BranchCondition) -> &mut Self {
        self.set_c(u16::from(branch));
        self
    }

    pub fn set_input(&mut self, input: InputMode) -> &mut Self {
        self.set_c(u16::from(input));
        self
    }

    pub fn set_shift(&mut self, shift: ShiftDirection) -> &mut Self {
        self.set_c(u16::from(shift));
        self
    }

    pub fn set_data(&mut self, data: i8) -> &mut Self {
        let data: u8 = data as u8; // intentional overflow cast this will use twos complement
        self.set_d(u16::from(data));
        self
    }

    pub fn finish(&mut self) -> u16 {
        self.0
    }
}

impl InstructionBuilder {
    //                    A___~B_~C_~D_______
    const ZERO_A: u16 = 0b0000_11_11_11111111;
    const ZERO_B: u16 = 0b1111_00_11_11111111;
    const ZERO_C: u16 = 0b1111_11_00_11111111;
    const ZERO_D: u16 = 0b1111_11_11_00000000;

    fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & Self::ZERO_A) | ((val << 12) & !Self::ZERO_A);
    }

    fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & Self::ZERO_B) | ((val << 10) & !Self::ZERO_B);
    }

    fn set_c(&mut self, val: u16) {
        self.0 = (self.0 & Self::ZERO_C) | ((val << 8) & !Self::ZERO_C);
    }

    fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & Self::ZERO_D) | (val & !Self::ZERO_D);
    }
}
