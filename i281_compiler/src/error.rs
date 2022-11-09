#[derive(Clone, Debug)]
pub enum StaticAnalysis {
    NonConstAddress,
    AddressOutOfBounds,
    UnknownVariable,
    TooManyRegisters,
}

#[derive(Clone, Debug)]
pub enum Error {
    StaticAnalysis(StaticAnalysis),
}

impl Error {
    pub fn address_oob() -> Self {
        Self::StaticAnalysis(StaticAnalysis::AddressOutOfBounds)
    }

    pub fn unknown_variable() -> Self {
        Self::StaticAnalysis(StaticAnalysis::UnknownVariable)
    }

    pub fn too_many_registers() -> Self {
        Self::StaticAnalysis(StaticAnalysis::TooManyRegisters)
    }
}
