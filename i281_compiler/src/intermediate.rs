pub enum IrRegister {
    A,
    B,
    C,
    D,
}

pub enum IrBranch {
    Always,
    Equal,
    Greater,
    GreaterEqual,
}

pub struct IrVariable {
    mem_addr: u8,
    values: Vec<i8>,
}

pub enum IrInstruction {
    NoOp,
    InputC { addr: u8, },
    InputCF { rx: IrRegister, addr: u8 },
    InputD { addr: u8 },
    InputDF { rx: IrRegister, addr: u8 },
    Move { rx: IrRegister, ry: IrRegister },
    LoadI { rx: IrRegister, ival: i8 },
    LoadP { rx: IrRegister, pval: i8 },
    Add { rx: IrRegister, ry: IrRegister },
    AddI { rx: IrRegister, ival: i8 },
    Sub { rx: IrRegister, ry: IrRegister },
    SubI { rx: IrRegister, ival: i8 },
    Load { rx: IrRegister, addr: u8 },
    LoadF { rx: IrRegister, ry: IrRegister, addr: u8 },
    Store { addr: u8, rx: IrRegister },
    StoreF {addr: u8, ry: IrRegister, rx: IrRegister },
    ShiftL { rx: IrRegister },
    ShiftR { rx: IrRegister },
    Cmp { rx: IrRegister, ry: IrRegister },
    Branch { cond: IrBranch, pcoff: i8 },
}
