use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Brain Fuck commands
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BfCommand {
    /// Comment, a character that is not a BF command
    Comment,
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
            BfCommand::Comment => '#',
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
            BfCommand::Comment => write!(f, "Comment"),
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

/// Location of a command in a BF program.
/// The point in the source file is comprised of the line number and the offset
/// within the line.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BfLocation {
    /// The line number in the source
    line: usize,
    /// The offset from the start of the line in the source
    offset: usize,
}

// Implementation details for locations in the source file of BF commands
impl BfLocation {
    /// Create a new BF location
    pub fn new(line: usize, offset: usize) -> Self {
        Self { line, offset }
    }

    /// The line number
    pub fn line(&self) -> usize {
        self.line
    }

    /// The offset within the line
    pub fn offset(&self) -> usize {
        self.offset
    }
}

/// Display a BF location for human consumption
impl fmt::Display for BfLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.offset)
    }
}

/// Brain Fuck instructions
///
/// Instructions consist of the BF command and the location in the file they were found at.
/// The location is used for future reference.
/// Typical usage is to create a vector of BfInstructions which will equate to a Brain Fuck program
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BfInstruction {
    /// The BF command that makes up the instruction
    command: BfCommand,
    /// The location in the source file where the command is
    location: BfLocation,
}

// Implementations for BfInstructions
impl BfInstruction {
    /// Create a new BF instruction.
    ///
    /// Usage:
    /// ``
    /// instructions.push(BfInstruction::new(command, line_no, char_pos));
    /// ``
    pub fn new(command: BfCommand, line: usize, offset: usize) -> Self {
        Self {
            command,
            location: BfLocation { line, offset },
        }
    }

    /// The BF command.
    pub fn command(&self) -> BfCommand {
        self.command
    }

    // The location (line and offset) the BF command was read from.
    pub fn location(&self) -> BfLocation {
        self.location
    }
}

impl fmt::Display for BfInstruction {
    /// Format the BF instruction for display and human consumption
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @{}", self.command, self.location,)
    }
}

/// Brain Fuck jumps
///
/// Jumps require matching up the start and end locations.
/// This structure is records the position of the jump forward and the
/// matching jump back.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BfJumpLocation {
    // Nesting depth of jumps (not used yet, maybe never
    #[allow(unused)]
    depth: usize,

    // Location of jump forward in the pair of jumps
    forward: BfLocation,

    // Location of jump back in the pair of jumps
    backward: BfLocation,
}

// Implementations for BfJumpLocation
impl BfJumpLocation {
    /// Create a new BF jump location.
    pub fn new(depth: usize, forward: BfLocation, backward: BfLocation) -> Self {
        Self {
            depth,
            forward,
            backward,
        }
    }

    /// Nesting depth for the pair of jumps
    // TODO: Not sure if it will be useful.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// The location of the jump forward
    pub fn forward(&self) -> BfLocation {
        self.forward
    }

    /// The location of the jump backward
    pub fn backward(&self) -> BfLocation {
        self.backward
    }
}

impl fmt::Display for BfJumpLocation {
    /// Format the BF location for display and human consumption
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Forward: {} Back: {} Depth: {}",
            self.forward, self.backward, self.depth
        )
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
    /// Locations of matching jumps
    jump_locations: Vec<BfJumpLocation>,
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

    /// The program's jumps
    pub fn jump_locations(&self) -> &Vec<BfJumpLocation> {
        &self.jump_locations
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
    /// Example:
    ///
    /// ```
    /// let filename:String = std::env::args().nth(1);
    /// let content = std::fs::read_to_string(&filename).unwrap();
    /// let program = bft_types::BfProgram::new(filename, &content).unwrap();
    /// ```
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
            jump_locations: Vec::new(),
        };
        Ok(program)
    }

    /// Read a BrainFuck program from a file. The program will be returned in a Result<>.
    /// If the file is not found or there are issues with reading it an error will be returned.
    ///
    /// Example:
    ///
    /// ```
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

    /// Validate a BrainFuck program by finding matching jump forwards and backs
    ///
    /// Parse the instructions using a stack to keep track of jumps and when a pair
    /// is found, place their locations in the source file in a vector. During the
    /// parsing the stack is checked as it should not be empty when a jump back
    /// is found, nor should it be empty when all instructions have been parsed.
    /// The former means an extra jump back, the later means an extra jump forward.
    ///
    /// Usage:
    /// ```
    ///   let mut program = bft_types::BfProgram::new(&"sample.bf", "[>]").unwrap();
    ///   if program.validate().is_ok() {
    ///     println!("Valid BF program, will now run it....");
    ///   }
    ///   let mut program = bft_types::BfProgram::new(&"sample.bf", "[]]").unwrap();
    ///   if program.validate().is_err() {
    ///     println!("Not a valid BF program");
    ///   }
    /// ```
    pub fn validate(&mut self) -> Result<(), anyhow::Error> {
        println!("Validating...");

        // Use a stack to keep track of pairs of jumps. The [ and ] in the BF code.
        // Jump forwards (the [) are pushed on to the stack. When a jump backward is
        // found, the top item on the stack is removed which will be the matching jump
        // forward. The locations of both jumps are then saved for later use
        let mut stack: Vec<BfInstruction> = Vec::new();

        // Parse the BF program and find the jumps
        for i in &self.instructions {
            if i.command == (BfCommand::JumpForward) {
                stack.push(*i);
            } else if i.command == (BfCommand::JumpBackward) {
                // If stack is empty, then the jump forward for this jump back
                // is missing. Or there is an extra jump back.
                if stack.is_empty() {
                    return Err(anyhow!("Extra {}", i));
                }

                // Make a note of the locations of the two jumps in the pair
                let last_jump = stack.pop().unwrap();
                self.jump_locations.push(BfJumpLocation {
                    depth: stack.len(),
                    forward: last_jump.location,
                    backward: i.location,
                });
            }
        }

        // If the stack is not empty, then there is a missing jump back or an
        // extra jump forward.
        if !stack.is_empty() {
            let last_bracket = stack.pop().unwrap();
            return Err(anyhow!("Extra {}", last_bracket));
        }

        // Stack is empty, all jumps paired up, so pass their locations back
        Ok(())
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
        assert_eq!(program.instructions()[0].location().line(), 1);
        assert_eq!(program.instructions()[0].location().offset(), 1);

        // Last command should be Output and is at line 2, offset 2
        let last_inst = program
            .instructions()
            .last()
            .expect("Invalid last instruction");
        assert_eq!(last_inst.command(), BfCommand::OutputValue);
        assert_eq!(last_inst.location().line(), 2);
        assert_eq!(last_inst.location().offset(), 2);
    }

    //
    // Check individual BF commands.
    //

    #[test]
    fn check_inc_data_pointer_command() {
        let program = BfProgram::new(&"sample.bf", ">").unwrap();

        assert_eq!(
            program.instructions()[0].command(),
            BfCommand::IncDataPointer
        );
        assert_eq!(program.instructions()[0].location().line(), 1);
        assert_eq!(program.instructions()[0].location().offset(), 1);
    }

    #[test]
    fn check_dec_data_pointer_command() {
        let program = BfProgram::new(&"sample.bf", "<").unwrap();

        assert_eq!(
            program.instructions()[0].command(),
            BfCommand::DecDataPointer
        );
        assert_eq!(program.instructions()[0].location().line(), 1);
        assert_eq!(program.instructions()[0].location().offset(), 1);
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
        assert_eq!(program.validate().is_ok(), true);
    }

    // Validate a bad BF program
    #[test]
    fn validate_extra_closing_bracket() {
        let mut program = BfProgram::new(&"bad.bf", "><+-.]").unwrap();
        assert_eq!(program.validate().is_err(), true);
    }

    // Validate a bad BF program
    #[test]
    fn validate_no_closing_bracket() {
        let mut program = BfProgram::new(&"bad.bf", "><+-[.").unwrap();
        assert_eq!(program.validate().is_err(), true);
    }

    // Validate a bad BF program
    #[test]
    fn validate_mismatched_bracket() {
        let mut program = BfProgram::new(&"bad.bf", "><+-].[").unwrap();
        assert_eq!(program.validate().is_err(), true);
    }

    // Validate a empty BF program
    #[test]
    fn validate_empty() {
        let mut program = BfProgram::new(&"empty.bf", "").unwrap();
        assert_eq!(program.validate().is_ok(), true);
    }

    // Check that the locations of jumps are correct in a good BF program
    #[test]
    fn validate_good_jumps() {
        let mut program = BfProgram::new(&"good.bf", "><+-[.]").unwrap();
        assert_eq!(program.validate().is_ok(), true);
        assert_eq!(program.jump_locations().len(), 1);
        assert_eq!(program.jump_locations()[0].forward.line(), 1);
        assert_eq!(program.jump_locations()[0].forward.offset(), 5);
        assert_eq!(program.jump_locations()[0].backward.line(), 1);
        assert_eq!(program.jump_locations()[0].backward.offset(), 7);
    }
}
