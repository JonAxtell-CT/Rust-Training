use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Brain Fuck commands
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BfCommand {
    /// Increment the data pointer (to point to the next cell to the right).
    IncDataPointer,
    /// Decrement the data pointer (to point to the next cell to the left).
    DecDataPointer,
    /// Increment (increase by one) the byte at the data pointer.
    IncValue,
    /// Decrement (decrease by one) the byte at the data pointer.
    DecValue,
    /// Output the byte at the data pointer.
    OutputValue,
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    InputValue,
    /// If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
    JumpForward,
    /// If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
    JumpBackward,
}

// Implementation details for Brain Fuck commands
impl BfCommand {
    /// Connvert a character to a BF command. An option is returned which
    /// will be none if the character is not a valid BF command.
    pub fn from_char(ch: char) -> Option<BfCommand> {
        match ch {
            '>' => Some(BfCommand::IncDataPointer),
            '<' => Some(BfCommand::DecDataPointer),
            '+' => Some(BfCommand::IncValue),
            '-' => Some(BfCommand::DecValue),
            '.' => Some(BfCommand::OutputValue),
            ',' => Some(BfCommand::InputValue),
            '[' => Some(BfCommand::JumpForward),
            ']' => Some(BfCommand::JumpBackward),
            _ => None,
        }
    }

    /// Convert a command back to a char if outputting the program
    pub fn to_char(cmd: BfCommand) -> char {
        match cmd {
            BfCommand::IncDataPointer => '>',
            BfCommand::DecDataPointer => '<',
            BfCommand::IncValue => '+',
            BfCommand::DecValue => '-',
            BfCommand::OutputValue => '.',
            BfCommand::InputValue => ',',
            BfCommand::JumpForward => '[',
            BfCommand::JumpBackward => ']',
        }
    }
}

/// Display a BF command in a very verbose manner for human consumption
impl fmt::Display for BfCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BfCommand::IncDataPointer => write!(f, "Increment data pointer"),
            BfCommand::DecDataPointer => write!(f, "Decrement data pointer"),
            BfCommand::IncValue => write!(f, "Increment byte at data pointer"),
            BfCommand::DecValue => write!(f, "Decrement byte at data pointer"),
            BfCommand::OutputValue => write!(f, "Output byte at data pointer"),
            BfCommand::InputValue => write!(f, "Input byte at data pointer"),
            BfCommand::JumpForward => write!(f, "Jump forward if zero"),
            BfCommand::JumpBackward => write!(f, "Jump backward if nonzero"),
        }
    }
}

/// Brain Fuck instructions.
/// Instructions consist of the BF command and the location in the file they were found at.
/// The location is used for future reference.
/// Typical usage is to create a vector of BfInstructions which will equate to a Brain Fuck program
#[derive(Debug, PartialEq)]
pub struct BfInstruction {
    /// The BF command that makes up the instruction
    command: BfCommand,
    /// The line number of the BF command in the source
    line_no: usize,
    /// The offset from the start of the line of the BF command in the source
    char_pos: usize,
}

// Implementations for BfInstructions
impl BfInstruction {
    /// Create a new BF instruction.
    ///
    /// Usage:
    /// ``
    /// instructions.push(BfInstruction::new(command, line_no, char_pos));
    /// ``
    pub fn new(command: BfCommand, line_no: usize, char_pos: usize) -> Self {
        Self {
            command,
            line_no,
            char_pos,
        }
    }

    /// The BF command.
    pub fn command(&self) -> BfCommand {
        self.command
    }

    // The line the BF command was read from.
    pub fn line_no(&self) -> usize {
        self.line_no
    }

    // The char position within the line the BF command was read from.
    pub fn char_pos(&self) -> usize {
        self.char_pos
    }
}

/// Brain Fuck program
///
/// To read a BF program from a file use the from_file method.
///
/// ``
/// let program =
///     bft_types::BfProgram::from_file(&env::args().nth(1).ok_or("You didn't specify a file")?)?;
/// ``
///
/// Use the instructions method to access the BF commands.
///
/// ``
/// for inst in program.instructions() {
///     println!("{:?}", inst);
/// }
/// ``
#[derive(Debug)]
pub struct BfProgram {
    /// The program file
    filename: PathBuf,
    /// The instructions parsed from the file
    instructions: Vec<BfInstruction>,
}

// Implementations for BfProgram
impl BfProgram {
    /// The filename the program was read from
    pub fn filename(&self) -> &Path {
        &self.filename
    }

    /// The program's instructions
    pub fn instructions(&self) -> &Vec<BfInstruction> {
        &self.instructions
    }

    /// Size of program
    pub fn size(&self) -> usize {
        self.instructions.len()
    }
}

// Implementation details for Brain Fuck program
impl BfProgram {
    /// Create a new Brain Fuck program from a string.
    /// The filename the contents were read from are passed as arguments so that
    /// it can be stored for future reference.
    ///
    /// The contents are parsed for BF instructions.
    ///
    /// Exmaple:
    ///
    /// ``
    /// let filename:String = std::env::args().nth(1);
    /// let content = std::fs::read_to_string(&filename).unwrap();
    /// let program = bft_types::BfProgram::new(filename, &content).unwrap();
    /// ``
    pub fn new(filename: impl AsRef<Path>, content: &str) -> std::io::Result<Self> {
        let mut instructions = Vec::new();
        let mut line_no = 1;
        for line in content.lines() {
            let mut char_pos = 1;
            for ch in line.chars() {
                if let Some(command) = BfCommand::from_char(ch) {
                    instructions.push(BfInstruction::new(command, line_no, char_pos));
                }
                char_pos += 1;
            }
            line_no += 1;
        }

        let program = Self {
            filename: filename.as_ref().to_path_buf(),
            instructions,
        };
        Ok(program)
    }

    /// Read a BrainFuck program from a file. The program will be returned in a Result<>.
    /// If the file is not found or there are issues with reading it an error will be returned.
    ///
    /// Example:
    ///
    /// ```no_run
    /// use bft_types::BfProgram;
    /// let program = BfProgram::from_file(&"hello-world.bf");
    /// for inst in program.expect("Opps").instructions() {
    ///    println!("{:?}", inst);
    /// }
    /// ```
    pub fn from_file(filename: impl AsRef<Path>) -> std::io::Result<BfProgram> {
        let content = fs::read_to_string(filename.as_ref())?;
        let program = BfProgram::new(filename, &content)?;
        Ok(program)
    }

    /// Validate a BrainFuck program
    ///
    /// Note: Currently just does a simplistic check of the jumps and makes sure
    /// they match in number.
    ///
    /// Todo: Make the check parse the jumps and find where the jump backs get
    /// out of step with the jump forwards. In other words when there is an extra
    /// jump back point out the location (line, offset).
    /// Eg. []] will error on offset 3
    ///
    /// Usage:
    /// ```
    ///   let mut program = bft_types::BfProgram::new(&"sample.bf", "[>]").unwrap();
    ///   if program.validate() {
    ///     println!("Valid BF program, will now run it....");
    ///   }
    ///   let mut program = bft_types::BfProgram::new(&"sample.bf", "[]]").unwrap();
    ///   if !program.validate() {
    ///     println!("Not a valid BF program");
    ///   }
    /// ```
    pub fn validate(&mut self) -> bool {
        println!("Validating...");
        let fwd = &self
            .instructions
            .iter()
            .filter(|i| i.command() == BfCommand::JumpForward)
            .count();
        // .collect::<Vec<&BfInstruction>>();
        let back = &self
            .instructions
            .iter()
            .filter(|i| i.command() == BfCommand::JumpBackward)
            .count();
        // .collect::<Vec<&BfInstruction>>();

        fwd == back

        // for i in &self.instructions {
        //       if i.command() == BfCommand::JumpForward || i.command() == BfCommand::JumpBackward {
        //           println!("{:?}", i);
        //       }
        //   }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that a small program is read and is the right size.
    #[test]
    fn minimal_program() {
        let program = BfProgram::new(&"tiny.bf", "><+-.").unwrap();
        assert_eq!(program.size(), 5);
    }

    // Test that commands in a BF program have been read correctly.
    #[test]
    fn check_commands_in_minimal_program() {
        let program = BfProgram::new(&"sample.bf", "><+\n-.").unwrap();

        // First command should be Inc and is at line 1, offset 1
        assert_eq!(
            program.instructions()[0].command(),
            BfCommand::IncDataPointer
        );
        assert_eq!(program.instructions()[0].line_no(), 1);
        assert_eq!(program.instructions()[0].char_pos(), 1);

        // Last command should be Output and is at line 2, offset 2
        let last_inst = program
            .instructions()
            .last()
            .expect("Invalid last instruction");
        assert_eq!(last_inst.command(), BfCommand::OutputValue);
        assert_eq!(last_inst.line_no(), 2);
        assert_eq!(last_inst.char_pos(), 2);
    }

    /*
     * Check individual BF commands.
     */
    #[test]
    fn check_inc_data_pointer_command() {
        let program = BfProgram::new(&"sample.bf", ">").unwrap();

        assert_eq!(
            program.instructions()[0].command(),
            BfCommand::IncDataPointer
        );
        assert_eq!(program.instructions()[0].line_no(), 1);
        assert_eq!(program.instructions()[0].char_pos(), 1);
    }

    #[test]
    fn check_dec_data_pointer_command() {
        let program = BfProgram::new(&"sample.bf", "<").unwrap();

        assert_eq!(
            program.instructions()[0].command(),
            BfCommand::DecDataPointer
        );
        assert_eq!(program.instructions()[0].line_no(), 1);
        assert_eq!(program.instructions()[0].char_pos(), 1);
    }

    #[test]
    fn check_inc_value_command() {
        assert_eq!(BfCommand::from_char('+'), Some(BfCommand::IncValue));
    }

    #[test]
    fn check_dec_value_command() {
        assert_eq!(BfCommand::from_char('-'), Some(BfCommand::DecValue));
    }

    #[test]
    fn check_output_byte_command() {
        assert_eq!(BfCommand::from_char('.'), Some(BfCommand::OutputValue));
    }

    #[test]
    fn check_input_byte_command() {
        assert_eq!(BfCommand::from_char(','), Some(BfCommand::InputValue));
    }

    #[test]
    fn check_jump_forward_command() {
        assert_eq!(BfCommand::from_char('['), Some(BfCommand::JumpForward));
    }

    #[test]
    fn check_jump_backward_command() {
        assert_eq!(BfCommand::from_char(']'), Some(BfCommand::JumpBackward));
    }

    #[test]
    fn check_invalid_command() {
        assert_eq!(BfCommand::from_char('X'), None);
    }

    // Check that non BF characters in the source file are skipped.
    #[test]
    fn ignore_non_bf_commands() {
        let program = BfProgram::new(&"not-bf.txt", "This is not a BF program").unwrap();
        assert_eq!(program.size(), 0);
    }

    // Check that an empty file is handled.
    #[test]
    fn read_empty_file() {
        let program = BfProgram::new(&"empty-file.bf", "").unwrap();
        assert_eq!(program.size(), 0);
    }

    // Validate a good BF program
    #[test]
    fn validate_good() {
        let mut program = BfProgram::new(&"good.bf", "><+-[.]").unwrap();
        assert_eq!(program.validate(), true);
    }

    // Validate a good BF program
    #[test]
    fn validate_bad() {
        let mut program = BfProgram::new(&"bad.bf", "><+-[.").unwrap();
        assert_eq!(program.validate(), false);
    }
}
