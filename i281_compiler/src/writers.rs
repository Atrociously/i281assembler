use std::io::Write;

use crate::instruction_builder::{InstructionBuilder, BinOpcode, BinRegister};

use i281_ast::{Instruction, Literal, Variable};

pub trait CodeWriter {
    fn write_ins<W: Write>(
        &mut self,
        out: &mut W,
        instruction: Instruction,
    ) -> Result<(), std::io::Error>;
}

pub trait DataWriter {
    fn write_var<W: Write>(
        &mut self,
        out: &mut W,
        variable: Variable,
    ) -> Result<(), std::io::Error>;
}

#[derive(Clone, Copy, Default)]
pub struct VerilogWriter {
    index: usize,
}

impl CodeWriter for VerilogWriter {
    fn write_ins<W: Write>(
        &mut self,
        out: &mut W,
        instruction: Instruction,
    ) -> Result<(), std::io::Error> {
        let mut ins = InstructionBuilder::new();

        match &instruction {
            Instruction::NoOp => {
                ins.set_opcode(BinOpcode::NOOP);
            }
            Instruction::InputC(_addr) => {
                ins.set_opcode(BinOpcode::INPUT);
                ins.set_ry(BinRegister::A);
            }
            Instruction::Store(addr, reg) => {
                ins.set_opcode(BinOpcode::STORE);
            },
            _ => {
                ins.set_opcode(BinOpcode::JUMP);
                ins.set_rx(BinRegister::B);
                ins.set_data(5);
            }
        }
        writeln!(out, "{:?} {:016b}", instruction, ins)
    }
}

impl DataWriter for VerilogWriter {
    fn write_var<W: Write>(
        &mut self,
        out: &mut W,
        variable: Variable,
    ) -> Result<(), std::io::Error> {
        fn write_simple<W: Write>(out: &mut W, lit: &Literal) -> Result<(), std::io::Error> {
            let val = match lit {
                Literal::Byte(ref b) => b.0,
                Literal::NotSet(_) => 0,
                Literal::Array(_) => unreachable!(),
            };
            write!(out, "8'b{:08b}", val)
        }

        match variable.value {
            Literal::Byte(..) => {
                write!(out, "{:?} ", variable.ident)?;
                write_simple(out, &variable.value)?;
                writeln!(out)
            }
            Literal::Array(ref arr) => {
                write!(out, "{:?} ", variable.ident)?;
                for b in arr.0.iter() {
                    write_simple(out, &b)?;
                    write!(out, ", ")?;
                }
                writeln!(out)
            }
            Literal::NotSet(..) => {
                write!(out, "{:?}", variable.ident)?;
                write_simple(out, &variable.value)?;
                writeln!(out)
            }
        }?;
        self.index += 1;

        Ok(())
    }
}
