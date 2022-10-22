use std::collections::HashMap;

use i281_ast::{Variable, Ident, Address};

use crate::error::Error;

pub struct StaticAnalyzer<'a> {
    vars: &'a HashMap<Ident, Variable>,
}

impl<'a> StaticAnalyzer<'a> {
    pub fn new(vars: &'a HashMap<Ident, Variable>) -> Self {
        Self {
            vars
        }
    }

    pub fn analyze_instruction<'f>(addr: Address) -> Result<(), Error<'f>> {
        todo!()
    }
}
