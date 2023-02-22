use std::env;
use std::error::Error;
//use std::fmt;
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

    // // Convert enum back to a char
    // fn to_char(cmd: BfCommands) -> char {
    //     match cmd {
    //         BfCommands::IncDataPointer => '>',
    //         BfCommands::DecDataPointer => '<',
    //         BfCommands::IncValue => '+',
    //         BfCommands::DecValue => '-',
    //         BfCommands::OutputValue => '.',
    //         BfCommands::InputValue => ',',
    //         BfCommands::JumpForward => '[',
    //         BfCommands::JumpBackward => ']',
    //     }
    // }

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

// impl fmt::Display for BfCommands {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

/*
 * Brain Fuck instructions
 */
#[derive(Debug)]
struct BfInstruction {
    command: BfCommands,
    line_no: usize,
    char_pos: usize,
}

#[derive(Debug)]
struct BfProgram {
    filename: String,
    instructions: Vec<BfInstruction>,
}

fn parse_bf_file(filename: &String) -> Result<Vec<BfInstruction>, Box<dyn std::error::Error>> {
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
    // #[cfg(debug_assertions)]
    // println!("{:?}", program);

    Ok(program)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut program = BfProgram {
        filename: env::args().nth(1).ok_or("You didn't specify a file")?,
        instructions: Vec::<BfInstruction>::new(),
    };
    program.instructions = parse_bf_file(&program.filename.to_string())?;

    for inst in &program.instructions {
        // println!("{:?}", inst);
        println!(
            "{} {} {}",
            inst.line_no,
            inst.char_pos,
            BfCommands::to_string(inst.command)
        );
    }
    println!("{:?}", program);
    Ok(())
}
