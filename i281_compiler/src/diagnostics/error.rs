use i281_ast::{Ident, Instruction};

#[derive(Clone, Debug, miette::Diagnostic, thiserror::Error)]
pub enum Error {
    #[error("too much user data was defined amount: `{found}` maximum allowed: `{max}`")]
    TooMuchUserData { found: usize, max: usize },
    #[error("there are too many code instructions: `{found}` maximum allowed: `{max}`")]
    TooManyInstructions { found: usize, max: usize },
    #[error("variable `{name}` was not found in user defined data for instruction: `{ins}`")]
    VariableNotFound { name: Ident, ins: Instruction },
    #[error("label `{name}` was not found")]
    LabelNotFound { name: Ident, ins: Instruction },
    #[error("address requires a register to be specified: `{ins}`")]
    AddressMissingRegister { ins: Instruction },
    #[error("address is not allowed to have a register specified: `{ins}`")]
    AddressWithRegister { ins: Instruction },
    #[error("address specified with multiple registers when only one is allowed: `{ins}`")]
    AddressTooManyRegisters { ins: Instruction },
    #[error("address value is out of bounds: `{ins}`")]
    AddressOOB { ins: Instruction },
}
