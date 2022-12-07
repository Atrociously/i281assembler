use std::collections::HashSet;

use i281_ast::{
    literal::Byte, Address, AddressItem, Ident, Instruction, Label, Oper, Register, Root, Variable,
};
use i281_ir::{BranchCondition, Instruction::*, Ir, ShiftDirection};

use crate::{
    diagnostics::{Diagnostic, Error, Failure, Result, Warning},
    CODE_ADDR_MAX, DATA_ADDR_MAX, DATA_ADDR_MIN,
};

pub struct Analyzer {
    variables: Vec<Variable>,
    labels: Vec<Label>,
    instructions: Vec<Instruction>,
    diagnostics: Vec<Diagnostic>,
}

impl Analyzer {
    pub fn new(ast: Root) -> Self {
        let variables = ast.data.map(|data| data.variables).unwrap_or_else(Vec::new);

        let labels = ast.code.labels;
        let instructions = ast.code.instructions;

        Self {
            variables,
            labels,
            instructions,
            diagnostics: Vec::new(),
        }
    }

    pub fn validate(mut self) -> Result<(Ir, Vec<Diagnostic>)> {
        self.validate_labels()?;
        let variables = self.validate_variables()?;

        if self.instructions.len() > CODE_ADDR_MAX as usize {
            self.diagnostics.push(
                Error::TooManyInstructions {
                    found: self.instructions.len(),
                    max: CODE_ADDR_MAX as usize,
                }
                .into(),
            );
        }

        let mut instructions = Vec::with_capacity(self.instructions.len());
        for (index, ins) in self.instructions.clone().iter().enumerate() {
            match self.build_instruction(index, &ins) {
                Ok(ins) => instructions.push(ins),
                Err(Failure::Skip) => continue,
                Err(other) => return Err(other),
            }
        }

        Ok((
            Ir {
                variables,
                instructions,
            },
            self.diagnostics,
        ))
    }

    fn get_label_offset(
        &mut self,
        current_index: usize,
        ident: &Ident,
        ins: &Instruction,
    ) -> Result<i8> {
        let current_index: i8 = current_index.try_into().expect("this should always be valid as there must never be more than 32 user code instructions");
        let label = self
            .labels
            .iter()
            .find(|l| &l.ident == ident)
            .ok_or_else(|| {
                self.diagnostics.push(
                    Error::LabelNotFound {
                        name: ident.clone(),
                        ins: ins.clone(),
                    }
                    .into(),
                );
                Failure::Skip
            })?;
        let label_index: i8 = label.code_addr.try_into().expect("this should always be valid as there must never be more than 32 user code instructions");
        Ok(label_index - current_index - 1)
    }

    fn get_var_address(&mut self, ident: &Ident, ins: &Instruction) -> Result<i8> {
        let var = self
            .variables
            .iter()
            .find(|v| &v.ident == ident)
            .ok_or_else(|| {
                self.diagnostics.push(
                    Error::VariableNotFound {
                        name: ident.clone(),
                        ins: ins.clone(),
                    }
                    .into(),
                );
                Failure::Skip
            })?;
        let var_addr: i8 = var.data_addr.try_into().expect("this should always be valid as there must never be more than 16 user data addresses assigned");
        Ok(var_addr)
    }

    fn get_address_value(&mut self, address: &Address, ins: &Instruction) -> Result<i8> {
        if !address.to.is_const() {
            self.diagnostics
                .push(Error::AddressWithRegister { ins: ins.clone() }.into());
            return Err(Failure::Skip);
        }

        let address = address
            .to
            .iter()
            .try_fold((0i8, None), |(mut addr, oper), (item, next_oper)| {
                let value = match item {
                    AddressItem::Var(v) => self.get_var_address(v, ins)?,
                    AddressItem::Lit(b) => b.0,
                    AddressItem::Reg(_) => unreachable!(),
                };
                match oper {
                    Some(Oper::Add) => match addr.checked_add(value) {
                        Some(v) => addr = v,
                        None => {
                            self.diagnostics
                                .push(Error::AddressOOB { ins: ins.clone() }.into());
                            return Err(Failure::Skip);
                        }
                    },
                    Some(Oper::Sub) => match addr.checked_sub(value) {
                        Some(v) => addr = v,
                        None => {
                            self.diagnostics
                                .push(Error::AddressOOB { ins: ins.clone() }.into());
                            return Err(Failure::Skip);
                        }
                    },
                    _ => addr = value,
                }
                Result::Ok((addr, next_oper.copied()))
            })?
            .0;

        if address < DATA_ADDR_MIN || address > DATA_ADDR_MAX {
            self.diagnostics
                .push(Error::AddressOOB { ins: ins.clone() }.into());
            return Err(Failure::Skip);
        }

        Ok(address)
    }

    fn get_address_value_with_offset(
        &mut self,
        address: &Address,
        ins: &Instruction,
    ) -> Result<(i8, Register)> {
        if address.to.is_const() {
            self.diagnostics
                .push(Error::AddressMissingRegister { ins: ins.clone() }.into());
            return Err(Failure::Skip);
        }

        // if there is more than one register specified in the address we have a problem
        if address
            .to
            .iter()
            .filter(|(i, _)| i.as_reg().is_some())
            .skip(1)
            .next()
            .is_some()
        {
            self.diagnostics
                .push(Error::AddressTooManyRegisters { ins: ins.clone() }.into());
            return Err(Failure::Skip);
        }

        // unwrap is safe because we check is_const before
        let register = address
            .to
            .iter()
            .find_map(|(item, _)| item.as_reg().copied())
            .unwrap();

        let address = address
            .to
            .iter()
            .try_fold((0i8, None), |(mut addr, oper), (item, next_oper)| {
                let value = match item {
                    AddressItem::Var(v) => self.get_var_address(v, ins)?,
                    AddressItem::Lit(b) => b.0,
                    AddressItem::Reg(_) => 0,
                };
                match oper {
                    Some(Oper::Add) => match addr.checked_add(value) {
                        Some(v) => addr = v,
                        None => {
                            self.diagnostics
                                .push(Error::AddressOOB { ins: ins.clone() }.into());
                            return Err(Failure::Skip);
                        }
                    },
                    Some(Oper::Sub) => match addr.checked_sub(value) {
                        Some(v) => addr = v,
                        None => {
                            self.diagnostics
                                .push(Error::AddressOOB { ins: ins.clone() }.into());
                            return Err(Failure::Skip);
                        }
                    },
                    _ => addr = value,
                }
                Result::Ok((addr, next_oper.copied()))
            })?
            .0;

        if address < DATA_ADDR_MIN || address > DATA_ADDR_MAX {
            self.diagnostics
                .push(Warning::AddressOOBPossible(ins.clone()).into());
        }

        Ok((address, register))
    }

    fn validate_variables(&mut self) -> Result<Vec<i281_ir::Variable>> {
        let vars_end = self
            .variables
            .last()
            .map(|v| v.data_addr + v.value.size_of());
        if let Some(found) = vars_end.filter(|end| *end > DATA_ADDR_MAX as usize) {
            self.diagnostics.push(
                Error::TooMuchUserData {
                    found,
                    max: DATA_ADDR_MAX as usize,
                }
                .into(),
            );
            // push a diagnostic but do nothing as we want to continue compiling
        };

        let mut unique = HashSet::with_capacity(self.variables.len());
        let mut vars = Vec::with_capacity(self.variables.len());
        for var in self.variables.iter() {
            if !unique.insert(&var.ident) {
                return Err(Failure::NonUniqueVariable(var.clone()));
            }
            vars.push(i281_ir::Variable::from(var));
        }

        Ok(vars)
    }

    fn validate_labels(&mut self) -> Result<()> {
        let mut unique = HashSet::new();
        for label in self.labels.iter() {
            if !unique.insert(&label.ident) {
                return Err(Failure::NonUniqueLabel(label.clone()));
            }
        }
        Ok(())
    }

    fn build_instruction(
        &mut self,
        index: usize,
        ins: &Instruction,
    ) -> Result<i281_ir::Instruction> {
        Ok(match ins {
            Instruction::NoOp => NoOp,
            Instruction::InputC(addr) => InputC {
                code_addr: self.get_address_value(addr, ins)?,
            },
            Instruction::InputCF(addr) => {
                let (code_offset, rx) = self.get_address_value_with_offset(addr, ins)?;
                InputCF {
                    rx: rx.into(),
                    code_offset,
                }
            }
            Instruction::InputD(addr) => InputD {
                data_addr: self.get_address_value(addr, ins)?,
            },
            Instruction::InputDF(addr) => {
                let (data_offset, rx) = self.get_address_value_with_offset(addr, ins)?;
                InputDF {
                    rx: rx.into(),
                    data_offset,
                }
            }
            Instruction::Move(rx, ry) => Move {
                rx: rx.into(),
                ry: ry.into(),
            },
            Instruction::LoadI(rx, Byte(val)) => LoadI {
                rx: rx.into(),
                value: *val,
            },
            Instruction::LoadP(_, _) => {
                unimplemented!("the compiler is not yet built to handle pointer operations")
            }
            Instruction::Add(rx, ry) => Add {
                rx: rx.into(),
                ry: ry.into(),
            },
            Instruction::AddI(rx, Byte(val)) => AddI {
                rx: rx.into(),
                value: *val,
            },
            Instruction::Sub(rx, ry) => Sub {
                rx: rx.into(),
                ry: ry.into(),
            },
            Instruction::SubI(rx, Byte(val)) => SubI {
                rx: rx.into(),
                value: *val,
            },
            Instruction::Load(rx, addr) => Load {
                rx: rx.into(),
                data_addr: self.get_address_value(addr, ins)?,
            },
            Instruction::LoadF(rx, addr) => {
                let (data_offset, ry) = self.get_address_value_with_offset(addr, ins)?;
                LoadF {
                    rx: rx.into(),
                    ry: ry.into(),
                    data_offset,
                }
            }
            Instruction::Store(addr, rx) => Store {
                data_addr: self.get_address_value(addr, ins)?,
                rx: rx.into(),
            },
            Instruction::StoreF(addr, rx) => {
                let (data_offset, ry) = self.get_address_value_with_offset(addr, ins)?;
                StoreF {
                    ry: ry.into(),
                    data_offset,
                    rx: rx.into(),
                }
            }
            Instruction::ShiftL(rx) => Shift {
                dir: ShiftDirection::Left,
                rx: rx.into(),
            },
            Instruction::ShiftR(rx) => Shift {
                dir: ShiftDirection::Right,
                rx: rx.into(),
            },
            Instruction::Cmp(rx, ry) => Cmp {
                rx: rx.into(),
                ry: ry.into(),
            },
            Instruction::Jump(ident) => Jump {
                pc_offset: self.get_label_offset(index, ident, ins)?,
            },
            Instruction::BrE(ident) => Branch {
                cond: BranchCondition::Equal,
                pc_offset: self.get_label_offset(index, ident, ins)?,
            },
            Instruction::BrNE(ident) => Branch {
                cond: BranchCondition::NotEqual,
                pc_offset: self.get_label_offset(index, ident, ins)?,
            },
            Instruction::BrG(ident) => Branch {
                cond: BranchCondition::Greater,
                pc_offset: self.get_label_offset(index, ident, ins)?,
            },
            Instruction::BrGE(ident) => Branch {
                cond: BranchCondition::GreaterEqual,
                pc_offset: self.get_label_offset(index, ident, ins)?,
            },
        })
    }
}
