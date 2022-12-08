# i281 Assembler
This is the repository for the rust i281 assembler which is an assembler suite for the i281 CPU.
It uses a parser-combinator strategy to parse the assembly language into an AST (Abstract Syntax Tree).
After which it sends the AST to an analyzer that checks for correctness and creates a much simpler IR (Intermediate Representation)
wich has a lot of the uneccessary data trimmed out of it. Then the compiler will take that IR and write it out in verilog format.

## Project structure
The project is structured into several components:
- Abstract Syntax Tree / Parser
- Intermediate Representation
- Analysis / Compilation functions
- Command Line Interface

The separation of these components makes the compiler more pluggable for using in other rust projects / partial compilation.
There may be goals of making a crate that is a web assembly library for use in web or other languages / projects.

## How to use
1. First check the github releases page for pre-compiled binaries of the compiler for your os, or compile from source
2. Once you have an executable you can run `compile281 --help` to get a list of options for the compiler command
3. To compile an assembly project run `compile281 [YOUR ASSEMBLY FILE]` and the results will be in the `./build/` directory

## Compiling From Source
1. Make sure you have rust [installed](https://www.rust-lang.org/tools/install).
2. Clone the repo to your local machine or unzip it from the archive
3. Run `cargo build --release` in the project directory
4. Your compiled binary will be located in *target/release/compile281*

## Other Info
- To run unit tests use `cargo test --workspace` to run all tests within the workspace
- The `examples` directory has as many assembly examples as I had access to
- Use [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) to make sure all libraries used are MIT licensed

## Libraries used and reasons
- [nom](https://github.com/Geal/nom) it provides a foundation for writing parser-combinators
- [nom_locate](https://github.com/fflorent/nom_locate) it provides some better tracking of parser location (for better errors)
- [serde](https://github.com/serde-rs/serde) it allows simple serialization and deserialization of rust data structures
- [serde_json](https://github.com/serde-rs/json) it uses serde to serialize to json
- [clap](https://docs.rs/clap/latest/clap/) it makes creation of a cli tool easy
- [thiserror](https://github.com/dtolnay/thiserror) allows simple creation of custom error types
- [miette](https://docs.rs/miette/latest/miette/) makes error diagnostics easier and pretty
- [paste](https://github.com/dtolnay/paste) used to generate test names for opcodes and maybe more tests in the future
