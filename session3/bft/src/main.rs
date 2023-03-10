use std::env;

#[allow(dead_code)]
const DEBUG: bool = true; // Set to true to enable debugging code

/// Program to read a Brain Fuck program and run it
/// Usage:
///     bft <filename.bf>
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program =
        bft_types::BfProgram::from_file(&env::args().nth(1).ok_or("You didn't specify a file")?)?;

    if DEBUG {
        // Debug code to dump BF program.
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

    // Create a tape for the program to be used by the interpreter
    let tape: bft_interp::BfTape<u8> =
        bft_interp::BfTape::new(30000, bft_interp::AllocStrategy::TapeIsFixed);
    tape.interpreter(&program);

    Ok(())
}
