#![allow(unused)]
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/*
 * Brain Fuck commands
 */
#[derive(Copy, Clone, Debug)]
enum BfCommands {
    IncDataPointer,
    DecDataPointer,
    IncValue,
    DecValue,
    OutputValue,
    InputValue,
    JumpForward,
    JumpBackward,
    Invalid,
}

/*
 * Implementation details for Brain Fuck commands
 */
impl BfCommands {
    // Convert char from BF source into enum
    fn from_char(ch: char) -> BfCommands {
        match ch {
            '>' => BfCommands::IncDataPointer,
            '<' => BfCommands::DecDataPointer,
            '+' => BfCommands::IncValue,
            '-' => BfCommands::DecValue,
            '.' => BfCommands::OutputValue,
            ',' => BfCommands::InputValue,
            '[' => BfCommands::JumpForward,
            ']' => BfCommands::JumpBackward,
            _ => BfCommands::Invalid,
        }
    }

    // Convert enum back to a char
    fn to_char(cmd: BfCommands) -> char {
        match cmd {
            BfCommands::IncDataPointer => '>',
            BfCommands::DecDataPointer => '<',
            BfCommands::IncValue => '+',
            BfCommands::DecValue => '-',
            BfCommands::OutputValue => '.',
            BfCommands::InputValue => ',',
            BfCommands::JumpForward => '[',
            BfCommands::JumpBackward => ']',
            _ => '?',
        }
    }

    // Convert enum to a string for human readable output
    fn to_string(cmd: BfCommands) -> String {
        match cmd {
            BfCommands::IncDataPointer => "Increment data pointer".to_string(),
            BfCommands::DecDataPointer => "Decrement data pointer".to_string(),
            BfCommands::IncValue => "Increment byte at data pointer".to_string(),
            BfCommands::DecValue => "Decrement byte at data pointer".to_string(),
            BfCommands::OutputValue => "Output byte at data pointer".to_string(),
            BfCommands::InputValue => "Input byte at data pointer".to_string(),
            BfCommands::JumpForward => "Jump forward if zero".to_string(),
            BfCommands::JumpBackward => "Jump backward if nonzero".to_string(),
            _ => "Invalid".to_string(),
        }
    }
}

// Explcit implementation of partialeq
impl PartialEq for BfCommands {
    fn eq(&self, rhs: &BfCommands) -> bool {
        matches!(
            (self, rhs),
            (Self::IncDataPointer, Self::IncDataPointer)
                | (Self::DecDataPointer, Self::DecDataPointer)
                | (Self::IncValue, Self::IncValue)
                | (Self::DecValue, Self::DecValue)
                | (Self::OutputValue, Self::OutputValue)
                | (Self::InputValue, Self::InputValue)
                | (Self::JumpForward, Self::JumpForward)
                | (Self::JumpBackward, Self::JumpBackward)
                | (Self::Invalid, Self::Invalid)
        )
    }
}

/*
 * Brain Fuck instructions
 */
#[derive(PartialEq)]
struct BfInstruction {
    command: BfCommands,
    line_no: usize,
    char_pos: usize,
}

// Human readable output
impl fmt::Display for BfInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}@{}] {}",
            self.line_no,
            self.char_pos,
            BfCommands::to_string(self.command),
        )
    }
}

// Debug output
impl fmt::Debug for BfInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}@{}] {}",
            self.line_no,
            self.char_pos,
            BfCommands::to_char(self.command),
        )
    }
}

/*
 * Parse the scores file and collect all the lines into one of two different
 * structures within a enum and return a vector containing everything
 */
fn parse_bf_file(filename: String) -> Result<Vec<BfInstruction>, Box<dyn std::error::Error>> {
    // println!("Filename is {}", filename);

    let fd = File::open(&filename);
    if fd.is_err() {
        return Err(format!("Could not open file '{}'", &filename).into());
    }
    let fd = fd?;
    let buf = BufReader::new(fd);
    let mut program = Vec::<BfInstruction>::new();
    let mut line_no = 1;
    for line in buf.lines() {
        if line.is_err() {
            return Err(format!(
                "Problem reading from file '{}' at line {}",
                &filename, line_no
            )
            .into());
        }
        let line = line?;
        let mut char_pos = 1;
        for c in line.chars() {
            let command = BfCommands::from_char(c);
            let inst = BfInstruction {
                command,
                line_no,
                char_pos,
            };
            if inst.command != BfCommands::Invalid {
                program.push(inst);
            }
            char_pos += 1;
        }
        line_no += 1;
    }

    // Debug to dump the BF program
    #[cfg(debug_assertions)]
    println!("{:?}", program);

    Ok(program)
}

/*
 * Main
 * Read a file containing a Brain Fuck program, parse it, and then
 * print out the commands in a human readable format.
 */
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];

        let bf_program = parse_bf_file(filename.to_string())?;

        for inst in bf_program {
            println!("{}", inst);
        }
    } else {
        println!("You didn't specify a file");
    }
    Ok(())
}
