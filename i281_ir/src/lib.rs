#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod branch_condition;
mod builder;
mod input_mode;
mod instruction;
mod opcode;
mod register;
mod shift_direction;
mod variable;

pub use branch_condition::BranchCondition;
pub use input_mode::InputMode;
pub use instruction::Instruction;
pub use opcode::OpCode;
pub use register::Register;
pub use shift_direction::ShiftDirection;
pub use variable::Variable;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Ir {
    pub variables: Vec<Variable>,
    pub instructions: Vec<Instruction>,
}
