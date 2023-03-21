mod cli;

/// Program to read a Brain Fuck program and run it
/// Usage:
///     bft <filename.bf>
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = &cli::Args::new();

    let mut program = bft_types::BfProgram::from_file(args.program())?;

    // Debug code to dump BF program.
    if args.debug() {
        for inst in program.instructions() {
            // println!("{:?}", inst);
            println!(
                "[{}]: {} {} {}",
                program.filename().to_string_lossy(),
                inst.line_no(),
                inst.char_pos(),
                inst.command(),
            );
        }
    }

    // Do some validation before running the BF program
    if program.validate() {
        println!("Valid BF program, will now run it....");
    } else {
        println!("Not a valid BF program");
        return Ok(());
    }

    // Create a tape for the program to be used by the interpreter
    let tape: bft_interp::BfTape<u8> =
        bft_interp::BfTape::new(args.cell_count(), args.extensible());
    tape.interpreter(&program);

    Ok(())
}
