use i281_ast::{Variable, Ident, Address, AddressItem, Literal, Oper};

use crate::error::{Error, StaticAnalysis};

pub struct StaticAnalyzer<'a> {
    vars: Vec<(usize, &'a Variable)>,
}

impl<'a> StaticAnalyzer<'a> {
    pub const ADDRESS_MAX: usize = 63;

    pub fn new(vars: &'a [Variable]) -> Self {
        let vars = Self::var_addrs(vars);
        Self {
            vars
        }
    }

    fn var_addrs(vars: &[Variable]) -> Vec<(usize, &Variable)> {
        let mut addr_pos: usize = 0;
        let mut with_addr = Vec::with_capacity(vars.len());
        for var in vars {
            with_addr.push((addr_pos, var));
            match &var.value {
                Literal::Byte(..)
                | Literal::NotSet(..) => {
                    addr_pos += 1;
                },
                Literal::Array(arr) => {
                    addr_pos += arr.0.len();
                },
            }
        }
        with_addr
    }

    fn get_var_addr(&self, var: &Ident) -> Result<usize, Error> {
        self.vars.iter().find(|v| v.1.ident == *var).map(|v| v.0).ok_or_else(Error::unknown_variable)
    }

    pub fn analyze_const_address(&self, addr: &Address) -> Result<(), Error> {
        let extract_addr_pos = |item: &AddressItem| -> Result<usize, Error> {
            match item {
                AddressItem::Var(var) => self.get_var_addr(var),
                AddressItem::Lit(lit) => Ok(lit.0 as usize),
                _ => unreachable!(),
            }
        };

        if !addr.to.is_const() {
            return Err(Error::StaticAnalysis(StaticAnalysis::NonConstAddress))
        }

        let addr_pos = addr.to.iter().try_fold((0usize, None), |mut accum, (item, oper)| {
            let item_val = extract_addr_pos(item)?;
            match accum.1 {
                Some(Oper::Add) => accum.0 += item_val,
                Some(Oper::Sub) => accum.0 -= item_val,
                None => accum.0 = item_val,
            }
            accum.1 = oper.copied();
            Ok(accum)
        }).map(|v| v.0)?;

        if addr_pos <= Self::ADDRESS_MAX {
            Ok(())
        } else {
            Err(Error::address_oob())
        }
    }
}
