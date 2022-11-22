#![forbid(unsafe_code)]

mod analyzer;
mod diagnostics;
mod verilog;

pub use analyzer::Analyzer;
pub use verilog::compile_verilog;

pub const BLOCK_SIZE: i8 = 16; // the size of the blocks in the generated output
pub const DATA_ADDR_MIN: i8 = 0;
pub const DATA_ADDR_MAX: i8 = 16; // in java compiler this is 64 but really this should be 16
pub const CODE_ADDR_MIN: i8 = 0;
pub const CODE_ADDR_MAX: i8 = 32; // there are a maximum of 30 instructions in user code

pub fn analyze<W: std::io::Write>(
    out: &mut W,
    ast: i281_ast::Root,
) -> Result<i281_ir::Ir, miette::ErrReport> {
    use miette::IntoDiagnostic;
    let (ir, diagnostics) = Analyzer::new(ast).validate()?;

    if diagnostics.len() > 0 {
        let handler = miette::GraphicalReportHandler::new();
        let mut report = String::new();
        for diagnostic in diagnostics.iter() {
            handler
                .render_report(&mut report, diagnostic)
                .into_diagnostic()?;
        }
        out.write(report.as_bytes()).into_diagnostic()?;
        todo!();
    }

    Ok(ir)
}
