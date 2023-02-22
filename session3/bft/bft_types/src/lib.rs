use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

/*
 * Brain Fuck commands
 */
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
#[allow(unused_imports)]
pub enum BfCommands {
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
    #[allow(dead_code)]
    pub fn from_char(ch: char) -> Option<BfCommands> {
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
    // pub fn to_char(cmd: BfCommands) -> char {
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
    #[allow(dead_code)]
    pub fn to_string(cmd: BfCommands) -> String {
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
#[allow(dead_code)]
#[allow(unused_imports)]
pub struct BfInstruction {
    command: BfCommands,
    line_no: usize,
    char_pos: usize,
}

impl BfInstruction {
    pub fn command(&self) -> BfCommands {
        self.command
    }
    pub fn line_no(&self) -> usize {
        self.line_no
    }
    pub fn char_pos(&self) -> usize {
        self.char_pos
    }
}
/*
 * Brain Fuck program
 */
#[derive(Debug)]
#[allow(dead_code)]
#[allow(unused_imports)]
pub struct BfProgram {
    filename: String,
    instructions: Vec<BfInstruction>,
}

impl BfProgram {
    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn instructions(&self) -> &Vec<BfInstruction> {
        &self.instructions
    }

    pub fn insts(&self) -> &[BfInstruction] {
        &self.instructions
    }
}
/*
 * Implementation details for Brain Fuck program
 */
impl BfProgram {
    #[allow(dead_code)]
    pub fn from_file(filename: &dyn AsRef<Path>) -> std::io::Result<Vec<BfInstruction>> {
        let mut program = Vec::<BfInstruction>::new();
        // let fname = filename.as_ref().to_string_lossy();
        let fd = File::open(&filename)?; //.map_err(|e| format!("Could not open file '{fname}': {e}"))?;
        println!("{:#?}", filename.as_ref().display());

        let buf = BufReader::new(fd);
        let mut line_no = 1;
        for line in buf.lines() {
            let line = line?; //.map_err(|e| {
                              //format!("Problem reading from file '{fname}' at line {line_no}: {e}")
                              //})?;
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

        Ok(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
