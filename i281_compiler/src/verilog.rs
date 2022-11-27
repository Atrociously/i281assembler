use std::{borrow::Cow, io};

use i281_ir::{Instruction, Ir, Variable};

use crate::{BLOCK_SIZE, CODE_ADDR_MAX, DATA_ADDR_MAX};

pub struct VerilogOutput<'a, W> {
    name: Cow<'a, str>,
    to: &'a mut W,
}

impl<'a, W: io::Write> VerilogOutput<'a, W> {
    pub fn new(name: &'a str, to: &'a mut W) -> Self {
        Self {
            name: Cow::Borrowed(name),
            to,
        }
    }
}

impl<W: io::Write> io::Write for VerilogOutput<'_, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.to.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.to.flush()
    }
}

fn write_file<W, F>(out: &mut W, name: &str, size: usize, f: F) -> Result<(), io::Error>
where
    W: io::Write,
    F: FnOnce(&mut W) -> Result<(), io::Error>,
{
    file_header(out, name, size)?;
    f(out)?;
    file_footer(out)
}

fn file_header<W: io::Write>(out: &mut W, name: &str, bus_size: usize) -> Result<(), io::Error> {
    const BSL: i8 = BLOCK_SIZE - 1;
    let bus_size = bus_size - 1;
    write!(out, "module {name}(")?;
    for i in 0..BSL {
        write!(out, "b{i}I,")?;
    }
    writeln!(out, "b{BSL}I);")?;

    for i in 0..BLOCK_SIZE {
        writeln!(out, "output [{bus_size}:0] b{i}I;")?;
    }
    Ok(())
}

fn assign_op<W: io::Write>(out: &mut W, index: usize, bytes: u16) -> Result<(), io::Error> {
    writeln!(
        out,
        "assign b{index}I[{size_down}:0] = {size}'b{bytes:0size$b};",
        size = 16,
        size_down = 15
    )
}

fn assign_data<W: io::Write>(out: &mut W, index: usize, bytes: i8) -> Result<(), io::Error> {
    writeln!(
        out,
        "assign b{index}I[{size_down}:0] = {size}'b{bytes:0size$b};",
        size = 8,
        size_down = 7
    )
}

fn file_footer<W: io::Write>(out: &mut W) -> Result<(), io::Error> {
    writeln!(out, "endmodule")
}

pub fn compile_verilog<A, B, C>(
    (code_low, code_high, data): (VerilogOutput<A>, VerilogOutput<B>, VerilogOutput<C>),
    ir: Ir,
) -> Result<(), io::Error>
where
    A: io::Write,
    B: io::Write,
    C: io::Write,
{
    let insts = ir
        .instructions
        .into_iter()
        .map(Instruction::build)
        .chain(std::iter::repeat(0).take(CODE_ADDR_MAX as usize)) // fill with at least max amt
        .enumerate();
    let vars = ir
        .variables
        .into_iter()
        .flat_map(Variable::into_data)
        .chain(std::iter::repeat(0).take(DATA_ADDR_MAX as usize)) // fill with at least max amt
        .enumerate();

    write_file(code_low.to, &code_low.name, 16, |out| {
        for (index, ins) in insts.clone().take(BLOCK_SIZE as usize) {
            assign_op(out, index, ins)?;
        }
        Ok(())
    })?;

    write_file(code_high.to, &code_high.name, 16, |out| {
        for (index, ins) in insts.skip(BLOCK_SIZE as usize).take(BLOCK_SIZE as usize) {
            assign_op(out, index - BLOCK_SIZE as usize, ins)?;
        }
        Ok(())
    })?;

    write_file(data.to, &data.name, 8, |out| {
        for (index, var) in vars.take(BLOCK_SIZE as usize) {
            assign_data(out, index, var)?;
        }
        Ok(())
    })?;

    Ok(())
}
