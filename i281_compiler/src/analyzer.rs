use crate::error::{Error, StaticAnalysis};
use i281_ast::{Address, AddressItem, Ident, Label, Oper, Pointer, Variable};

pub struct StaticAnalyzer<'a> {
    vars: &'a [Variable],
    labels: &'a [Label],
}

impl<'a> StaticAnalyzer<'a> {
    pub const ADDRESS_MAX_EX: usize = 64;

    pub fn new(vars: &'a [Variable], labels: &'a [Label]) -> Self {
        Self { vars, labels }
    }

    fn get_var_addr(&self, var: &Ident) -> Result<usize, Error> {
        self.vars
            .iter()
            .find(|v| v.ident == *var)
            .map(|v| v.data_addr)
            .ok_or_else(Error::unknown_variable)
    }

    pub fn check_pointer_validity(&self, ptr: &Pointer) -> Result<(), Error> {
        let points_to = self.get_var_addr(&ptr.var)? + ptr.offset.unwrap_or(0);

        if points_to < Self::ADDRESS_MAX_EX {
            Ok(())
        } else {
            Err(Error::address_oob())
        }
    }

    pub fn check_address_validity(&self, addr: &Address) -> Result<(), Error> {
        let mut has_reg = None;
        for (item, _oper) in addr.to.iter() {
            match item {
                // check that address variables exists
                AddressItem::Var(var) => {
                    self.get_var_addr(var)?;
                }
                AddressItem::Reg(reg) if has_reg.is_none() => has_reg = Some(reg),
                AddressItem::Reg(_) => return Err(Error::too_many_registers()),
                _ => continue,
            };
        }
        Ok(())
    }

    // also checks validity of address as well as constness
    // doesn't have to check for registers as const addresses are not allowed to have registers
    pub fn check_address_constness(&self, addr: &Address) -> Result<(), Error> {
        let addr_points_to = |item: &AddressItem| -> Result<usize, Error> {
            match item {
                AddressItem::Var(var) => self.get_var_addr(var),
                AddressItem::Lit(lit) => Ok((lit.0 as u8).into()), // intended overflow wrapping
                _ => unreachable!(),
            }
        };

        if !addr.to.is_const() {
            return Err(Error::StaticAnalysis(StaticAnalysis::NonConstAddress));
        }

        let addr_pointer_pos = addr
            .to
            .iter()
            .try_fold((0usize, None), |mut accum, (item, oper)| {
                let item_val = addr_points_to(item)?;
                match accum.1 {
                    Some(Oper::Add) => accum.0 += item_val,
                    Some(Oper::Sub) => accum.0 -= item_val,
                    None => accum.0 = item_val,
                }
                accum.1 = oper.copied();
                Ok(accum)
            })
            .map(|v| v.0)?;

        if addr_pointer_pos < Self::ADDRESS_MAX_EX {
            Ok(())
        } else {
            Err(Error::address_oob())
        }
    }
}
