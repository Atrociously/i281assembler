use crate::{
    builder::InstructionBuilder, BranchCondition, InputMode, OpCode, Register, ShiftDirection,
};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "opcode", rename_all = "UPPERCASE"))]
pub enum Instruction {
    NoOp,
    InputC {
        code_addr: i8,
    },
    InputCF {
        rx: Register,
        code_offset: i8,
    },
    InputD {
        data_addr: i8,
    },
    InputDF {
        rx: Register,
        data_offset: i8,
    },
    Move {
        rx: Register,
        ry: Register,
    },
    LoadI {
        rx: Register,
        value: i8,
    },
    Add {
        rx: Register,
        ry: Register,
    },
    AddI {
        rx: Register,
        value: i8,
    },
    Sub {
        rx: Register,
        ry: Register,
    },
    SubI {
        rx: Register,
        value: i8,
    },
    Load {
        rx: Register,
        data_addr: i8,
    },
    LoadF {
        rx: Register,
        ry: Register,
        data_offset: i8,
    },
    Store {
        data_addr: i8,
        rx: Register,
    },
    StoreF {
        ry: Register,
        data_offset: i8,
        rx: Register,
    },
    Shift {
        dir: ShiftDirection,
        rx: Register,
    },
    Cmp {
        rx: Register,
        ry: Register,
    },
    Jump {
        pc_offset: i8,
    },
    Branch {
        cond: BranchCondition,
        pc_offset: i8,
    },
}

impl Instruction {
    pub fn build(self) -> u16 {
        let mut builder = InstructionBuilder::new();

        let opcode = self.opcode();
        builder.set_opcode(opcode);

        match self {
            Self::NoOp => &mut builder,
            Self::InputC { code_addr } => builder.set_input(InputMode::C).set_data(code_addr),
            Self::InputCF { rx, code_offset } => builder
                .set_input(InputMode::CF)
                .set_rx(rx)
                .set_data(code_offset),
            Self::InputD { data_addr } => builder.set_input(InputMode::D).set_data(data_addr),
            Self::InputDF { rx, data_offset } => builder
                .set_input(InputMode::DF)
                .set_rx(rx)
                .set_data(data_offset),
            Self::Move { rx, ry } => builder.set_rx(rx).set_ry(ry),
            Self::LoadI { rx, value } => builder.set_rx(rx).set_data(value),
            Self::Add { rx, ry } => builder.set_rx(rx).set_ry(ry),
            Self::AddI { rx, value } => builder.set_rx(rx).set_data(value),
            Self::Sub { rx, ry } => builder.set_rx(rx).set_ry(ry),
            Self::SubI { rx, value } => builder.set_rx(rx).set_data(value),
            Self::Load { rx, data_addr } => builder.set_rx(rx).set_data(data_addr),
            Self::LoadF {
                rx,
                ry,
                data_offset,
            } => builder.set_rx(rx).set_ry(ry).set_data(data_offset),
            Self::Store { data_addr, rx } => builder.set_rx(rx).set_data(data_addr),
            Self::StoreF {
                ry,
                data_offset,
                rx,
            } => builder.set_rx(rx).set_ry(ry).set_data(data_offset),
            Self::Shift { dir: mode, rx } => builder.set_shift(mode).set_rx(rx),
            Self::Cmp { rx, ry } => builder.set_rx(rx).set_ry(ry),
            Self::Jump { pc_offset } => builder.set_data(pc_offset),
            Self::Branch {
                cond: mode,
                pc_offset,
            } => builder.set_branch(mode).set_data(pc_offset),
        };

        builder.finish()
    }

    pub fn opcode(self) -> OpCode {
        match self {
            Self::NoOp => OpCode::NoOp,
            Self::InputC { .. }
            | Self::InputCF { .. }
            | Self::InputD { .. }
            | Self::InputDF { .. } => OpCode::Input,
            Self::Move { .. } => OpCode::Move,
            Self::LoadI { .. } => OpCode::LoadI,
            Self::Add { .. } => OpCode::Add,
            Self::AddI { .. } => OpCode::AddI,
            Self::Sub { .. } => OpCode::Sub,
            Self::SubI { .. } => OpCode::SubI,
            Self::Load { .. } => OpCode::Load,
            Self::LoadF { .. } => OpCode::LoadF,
            Self::Store { .. } => OpCode::Store,
            Self::StoreF { .. } => OpCode::StoreF,
            Self::Shift { .. } => OpCode::Shift,
            Self::Cmp { .. } => OpCode::Cmp,
            Self::Jump { .. } => OpCode::Jump,
            Self::Branch { .. } => OpCode::Branch,
        }
    }
}
