use std::{fs::OpenOptions, io, path::Path};

use i281_ir::{Instruction, Variable, Ir};

use crate::{BLOCK_SIZE, CODE_ADDR_MAX, DATA_ADDR_MAX};

fn write_file<W, F>(out: &mut W, name: &str, size: usize, f: F) -> Result<(), io::Error>
where
    W: std::io::Write,
    F: FnOnce(&mut W) -> Result<(), io::Error>,
{
    file_header(out, name, size)?;
    f(out)?;
    file_footer(out)
}

fn file_header<W: std::io::Write>(
    out: &mut W,
    name: &str,
    bus_size: usize,
) -> Result<(), io::Error> {
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

fn assign_op<W: std::io::Write>(out: &mut W, index: usize, bytes: u16) -> Result<(), io::Error> {
    writeln!(
        out,
        "assign b{index}I[{size_down}:0] = {size}'b{bytes:0size$b};",
        size = 16,
        size_down = 15
    )
}

fn assign_data<W: std::io::Write>(out: &mut W, index: usize, bytes: i8) -> Result<(), io::Error> {
    writeln!(
        out,
        "assign b{index}I[{size_down}:0] = {size}'b{bytes:0size$b};",
        size = 8,
        size_down = 7
    )
}

fn file_footer<W: std::io::Write>(out: &mut W) -> Result<(), io::Error> {
    writeln!(out, "endmodule")
}

pub fn compile_verilog(
    out_dir: impl AsRef<Path>,
    ir: Ir
) -> Result<(), io::Error> {
    let out_dir = out_dir.as_ref();
    let insts = ir.instructions
        .into_iter()
        .map(Instruction::build)
        .chain(std::iter::repeat(0).take(CODE_ADDR_MAX as usize))
        .enumerate();
    let vars = ir.variables
        .into_iter()
        .flat_map(Variable::into_data)
        .chain(std::iter::repeat(0).take(DATA_ADDR_MAX as usize))
        .enumerate();

    let user_code_low = &mut OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(out_dir.join("User_Code_Low.v"))?;
    let user_code_high = &mut OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(out_dir.join("User_Code_High.v"))?;

    let user_data = &mut OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(out_dir.join("User_Data.v"))?;

    write_file(user_code_low, "User_Code_Low", 16, |out| {
        for (index, ins) in insts.clone().take(BLOCK_SIZE as usize) {
            assign_op(out, index, ins)?;
        }
        Ok(())
    })?;

    write_file(user_code_high, "User_Code_High", 16, |out| {
        for (index, ins) in insts.skip(BLOCK_SIZE as usize).take(BLOCK_SIZE as usize) {
            assign_op(out, index - BLOCK_SIZE as usize, ins)?;
        }
        Ok(())
    })?;

    write_file(user_data, "User_Data", 8, |out| {
        for (index, var) in vars.take(BLOCK_SIZE as usize) {
            assign_data(out, index, var)?;
        }
        Ok(())
    })?;

    Ok(())
}
