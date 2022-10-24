use std::collections::HashMap;

use i281_ast::{Root, Instruction};

use crate::static_analysis::StaticAnalyzer;

pub fn compile_ast(root: Root) {
    let vars: Vec<_> = {
        let num_vars = root.data.as_ref().map(|d| d.variables.len()).unwrap_or(0);
        let mut variables = HashMap::with_capacity(num_vars);

        for var in root.data.map(|d| d.variables).unwrap_or_else(Vec::new) {
            if let Some(_exist) = variables.insert(var.ident.clone(), var) {
                // variable already existed we got a name clash
                panic!("variable name clash");
            }
        }
        variables.into_values().collect()
    };

    let analyzer = StaticAnalyzer::new(&vars);

    for instruction in root.code.instructions {
        match instruction {
            Instruction::NoOp => {},
            Instruction::InputC(addr) => {
                analyzer.analyze_const_address(&addr).unwrap();
            }
            Instruction::InputCF(_addr) => {
                // non const address
            }
            Instruction::InputD(addr) => {
                analyzer.analyze_const_address(&addr).unwrap();
            }
            Instruction::InputDF(_addr) => {
                // non const address
            }
            Instruction::Move(_rx, _ry) => {}
            _ => todo!()
        }
    }
}
