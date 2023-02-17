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
#[derive(Copy, Clone, Debug, PartialEq)]
enum BfCommands {
    IncDataPointer,
    DecDataPointer,
    IncValue,
    DecValue,
    OutputValue,
    InputValue,
    JumpForward,
    JumpBackward,
}

/*
 * Implementation details for Brain Fuck commands
 */
impl BfCommands {
    fn from_char(ch: char) -> Option<BfCommands> {
        match ch {
            '>' => Some(BfCommands::IncDataPointer),
            '<' => Some(BfCommands::DecDataPointer),
            '+' => Some(BfCommands::IncValue),
            '-' => Some(BfCommands::DecValue),
            '.' => Some(BfCommands::OutputValue),
            ',' => Some(BfCommands::InputValue),
            '[' => Some(BfCommands::JumpForward),
            ']' => Some(BfCommands::JumpBackward),
            _ => None,
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
        }
    }
}

impl fmt::Display for BfCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BfCommands::IncDataPointer => write!(f, "Increment data pointer"),
            BfCommands::DecDataPointer => write!(f, "Decrement data pointer"),
            BfCommands::IncValue => write!(f, "Increment byte at data pointer"),
            BfCommands::DecValue => write!(f, "Decrement byte at data pointer"),
            BfCommands::OutputValue => write!(f, "Output byte at data pointer"),
            BfCommands::InputValue => write!(f, "Input byte at data pointer"),
            BfCommands::JumpForward => write!(f, "Jump forward if zero"),
            BfCommands::JumpBackward => write!(f, "Jump backward if nonzero"),
        }
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

// // Human readable output
// impl fmt::Display for BfInstruction {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}:{}] {}",
//             self.line_no,
//             self.char_pos,
//             BfCommands::to_string(self.command),
//         )
//     }
// }

// Debug output
impl fmt::Debug for BfInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}@{}: {}",
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
    let fd = File::open(&filename).map_err(|e| format!("Could not open file '{filename}': {e}"))?;
    let buf = BufReader::new(fd);
    let mut program = Vec::<BfInstruction>::new();
    let mut line_no = 1;
    for line in buf.lines() {
        let line = line.map_err(|e| {
            format!("Problem reading from file '{filename}' at line {line_no}: {e}")
        })?;
        let mut char_pos = 1;
        for c in line.chars() {
            if let Some(command) = BfCommands::from_char(c) {
                program.push(BfInstruction {
                    command,
                    line_no,
                    char_pos,
                });
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
    let filename = env::args().nth(1).ok_or("You didn't specify a file")?;
    let bf_program = parse_bf_file(filename.to_string())?;

    for inst in bf_program {
        println!(
            "[{filename}:{}:{}] {}",
            inst.line_no, inst.char_pos, inst.command
        );
    }
    Ok(())
}
