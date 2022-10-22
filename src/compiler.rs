use std::collections::HashMap;

use i281_ast::{Root, Instruction, AddressExpr, AddressItem};

pub fn compile_ast(root: Root) {
    let vars = {
        let num_vars = root.data.as_ref().map(|d| d.variables.len()).unwrap_or(0);
        let mut variables = HashMap::with_capacity(num_vars);

        for var in root.data.map(|d| d.variables).unwrap_or_else(Vec::new) {
            if let Some(_exist) = variables.insert(var.ident.clone(), var) {
                // variable already existed we got a name clash
                panic!("variable name clash");
            }
        }
        variables
    };

    for instruction in root.code.instructions {
        match instruction {
            Instruction::NoOp => {},
            Instruction::InputC(addr) => {
                if !addr.to.is_const() {
                    panic!("inputc non const address")
                }
            }
            _ => todo!()
        }
    }
}
