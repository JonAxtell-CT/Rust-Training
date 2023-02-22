#[allow(unused_imports)]
use std::env;
use std::error::Error;
//use std::fmt;
// use std::fmt::Debug;
// use std::fs::File;
// use std::io::BufRead;
// use std::io::BufReader;
// use std::path::Path;

use bft_types;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut program = bft_types::BfProgram {
    //     filename: env::args().nth(1).ok_or("You didn't specify a file")?,
    //     instructions: Vec::<bft_types::BfInstruction>::new(),
    // };
    let instructions =
        bft_types::BfProgram::from_file(&env::args().nth(1).ok_or("You didn't specify a file")?)?;
    // program.instructions = parse_bf_file(&program.filename.to_string())?;

    for inst in &instructions {
        // println!("{:?}", inst);
        println!(
            "{} {} {}",
            inst.line_no(),
            inst.char_pos(),
            bft_types::BfCommands::to_string(inst.command())
        );
    }
    Ok(())
}
