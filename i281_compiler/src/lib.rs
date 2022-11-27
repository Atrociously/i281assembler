#![forbid(unsafe_code)]

mod analyzer;
mod diagnostics;
mod verilog;

pub use analyzer::Analyzer;
pub use diagnostics::{Diagnostic, Error, Failure, Warning};
pub use verilog::{compile_verilog, VerilogOutput};

pub const BLOCK_SIZE: i8 = 16; // the size of the blocks in the generated output
pub const DATA_ADDR_MIN: i8 = 0;
pub const DATA_ADDR_MAX: i8 = 16; // in java compiler this is 64 but really this should be 16
pub const CODE_ADDR_MIN: i8 = 0;
pub const CODE_ADDR_MAX: i8 = 32; // there are a maximum of 30 instructions in user code

pub fn analyze<W: std::io::Write>(
    diagnostic_out: &mut W,
    ast: i281_ast::Root,
) -> Result<i281_ir::Ir, miette::ErrReport> {
    use miette::IntoDiagnostic;
    let (ir, diagnostics) = Analyzer::new(ast).validate()?;

    // output any diagnosics to the specified output writer and if any of the diagnosics were
    // errors we return a failure indicating we encountered errors
    if diagnostics.len() > 0 {
        let handler = miette::GraphicalReportHandler::new();
        // TODO: add a handler for warnings that makes it print yellow or something
        let mut report = String::new();
        let mut found_error = false;
        for diagnostic in diagnostics.iter() {
            if matches!(diagnostic, Diagnostic::Error(..)) {
                found_error = true;
            }
            handler
                .render_report(&mut report, diagnostic)
                .into_diagnostic()?;
        }
        diagnostic_out.write(report.as_bytes()).into_diagnostic()?;
        if found_error {
            return Err(Failure::EncounteredError.into());
        }
    }

    Ok(ir)
}
