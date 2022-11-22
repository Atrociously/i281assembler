use i281_ast::Instruction;

#[derive(Clone, Debug, miette::Diagnostic, thiserror::Error)]
pub enum Warning {
    #[error("possible address out of bounds depending on value of register `{0:?}`")]
    AddressOOBPossible(Instruction),
}
