/// Program to read a Brain Fuck program and run it
///
fn run_bft(args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut program = bft_types::BfProgram::from_file(args.program())?;

    // Debug code to dump BF program.
    if args.debug() > cli::DebugLevelType::Verbose {
        for inst in program.instructions() {
            // println!("{:?}", inst);
            println!(
                "[{}]: {} {}",
                program.filename().to_string_lossy(),
                inst.location(),
                inst.command(),
            );
        }
    }

    // Do some validation before running the BF program
    match program.validate() {
        Err(e) => {
            return Err(e.into());
        }
        Ok(()) => {
            if args.debug() > cli::DebugLevelType::Information {
                println!("Jumps");
                for j in program.jump_locations() {
                    println!("{}", j);
                }
            }
            println!("Valid BF program, will now run it....");
        }
    }

    // Create a tape for the program to be used by the interpreter
    let mut tape: bft_interp::BfTape<u8> =
        bft_interp::BfTape::new(args.cell_count(), args.extensible());
    tape.set_debug(args.debug());
    tape.interpreter(&program);

    Ok(())
}

/// Main
///
/// Will terminate with an exit code of 1 if there was an error in the BF
/// program. An exit code of zero is used if the BF program was run with no
/// issues.
///
/// Various options can be specified. They are
/// * -c \<cells\>  - Specify the number of cells in the BF program's tape. The default is 30,000.
/// * -e          - Allows the tape to grow as necessary. If not specified the tape is fixed in size.
/// * -d          - Debug output. Multiple occurences of this option increase the amount of debug information that is output.
/// * -h          - Help
/// * -V          - Version
///
/// Usage:
///     bft <filename.bf> \[options\]
fn main() {
    let args = &cli::Args::new();
    match run_bft(args) {
        Ok(_) => {}
        Err(e) => {
            println!("bft: Error in {}, {}", args.program().to_string_lossy(), e);
            std::process::exit(1)
        }
    }
    std::process::exit(0)
}
