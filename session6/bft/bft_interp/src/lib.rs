use bft_types::BfProgram;
use cli::OutputFormat;
use std::io::{Read, Write};
use thiserror::Error;

const MAX_TAPE_SIZE: usize = 30000;

/// Errors that can be returned by functions that handle running the BF program.
///
#[derive(Error, Debug)]
pub enum BfError {
    /// Error to indicate when the data pointer was moved before the start of the tape
    #[error(
        "Data pointer moved before start of tape at {} {}",
        program_pointer,
        instruction
    )]
    DataPtrMovedBeforeStart {
        instruction: bft_types::BfInstruction,
        program_pointer: usize,
    },
    /// Error to indicate when the data pointer was moved after the end of the tape
    #[error(
        "Data pointer moved after end of tape at {} {}",
        program_pointer,
        instruction
    )]
    DataPtrMovedAfterEnd {
        instruction: bft_types::BfInstruction,
        program_pointer: usize,
    },
    /// Error to indicate when the program pointer was moved after the end of the program
    #[error(
        "Program pointer moved after end of program at {} {}",
        program_pointer,
        instruction
    )]
    ProgramPtrMovedAfterEnd {
        instruction: bft_types::BfInstruction,
        program_pointer: usize,
    },
    /// Error to indicate a problem with the brackets when jumping forward or back
    #[error("Issue with brackets at {}", program_pointer)]
    BracketNotFound { program_pointer: usize },
    /// Error the occurs when reading/writing using the input/output functionality of the tape
    #[error(
        "I/O error {} at {} {} {}",
        error_msg,
        filepath.display(),
        instruction,
        program_pointer
    )]
    IOError {
        error_msg: std::io::Error,
        filepath: std::path::PathBuf,
        instruction: bft_types::BfInstruction,
        program_pointer: usize,
    },
}

/// Trait for the cells in the tape that allows them to be incremented/decremented and for
/// the value held in the cell to be set or extracted.
///
pub trait CellKind: Default + Clone + Copy + std::fmt::Debug {
    /// Increment a data cell's value
    fn inc(&mut self) -> Self
    where
        Self: std::marker::Sized;
    /// Decrement a data cell's value
    fn dec(&mut self) -> Self
    where
        Self: std::marker::Sized;

    /// Convert the value of a data cell to a u8
    fn to_u8(&self) -> u8
    where
        Self: std::marker::Sized;

    /// Convert the value of a data cell from u8
    fn from_u8(value: u8) -> Self
    where
        Self: std::marker::Sized;
}

/// Implementation of the Trait for the cells using u8
///
impl CellKind for u8 {
    /// Increment a data cell's value
    fn inc(&mut self) -> Self {
        self.wrapping_add(1)
    }

    /// Decrement a data cell's value
    fn dec(&mut self) -> Self {
        self.wrapping_sub(1)
    }

    /// Convert the value of a data cell to a u8
    fn to_u8(&self) -> u8 {
        *self
    }

    /// Set the value of a data cell from u8
    fn from_u8(value: u8) -> Self {
        value
    }
}

/// A tape is a representation of a Brain Fuck program's data as it's being interpreted. The
/// tape consists of cells which are manipulated as the BF program is interpreted.
///
pub struct BfTape<'a, T> {
    /// The program pointer.
    program_pointer: usize,
    /// Reference to the BF program
    program: &'a BfProgram,
    /// The data pointer. This is not the instruction pointer.
    data_pointer: usize,
    /// Indicates if more memory can be allocated from it's initial size or if it is fixed
    alloc_strategy: cli::AllocStrategy,
    /// Output format
    output_format: cli::OutputFormat,
    /// The tape itself
    tape: Vec<T>,
    /// Newline flag
    newline: bool,
    /// Debug flag
    debug: cli::DebugLevelType,
}

/// Implementation of the BF program's tape
///
impl<'a, T: CellKind + std::fmt::Debug> BfTape<'a, T> {
    /// Create a new tape for BF instructions.
    ///
    /// The BF program is passed in for reference purposes.
    ///
    /// If the size is specified as zero, then the default size of 30,000 cells will be allocated.
    ///
    /// The allocation strategy can be set so that the tape can grow as needed or it can be fixed.
    pub fn new(
        program: &'a BfProgram,
        tape_size: usize,
        alloc_strategy: cli::AllocStrategy,
        output_format: cli::OutputFormat,
    ) -> Self {
        Self {
            program_pointer: 0,
            program,
            data_pointer: 0,
            alloc_strategy,
            output_format,
            tape: if tape_size == 0 {
                vec![Default::default(); MAX_TAPE_SIZE]
            } else {
                vec![Default::default(); tape_size]
            },
            newline: false,
            debug: cli::DebugLevelType::None,
        }
    }

    // Data pointer handling methods
    // #############################

    /// The data pointer
    pub fn data_pointer(&self) -> usize {
        self.data_pointer
    }

    /// Set data pointer to start of program
    pub fn reset_data_pointer(&mut self) {
        self.data_pointer = 0;
    }

    /// Length of data tape
    pub fn data_length(&self) -> usize {
        self.tape.len()
    }

    /// Moves the data pointer forward
    pub fn move_data_pointer_forward(&mut self) -> Result<(), BfError> {
        if self.data_pointer == self.tape.len() - 1 {
            // The data pointer is at the end of the tape, we can either abort the BF program
            // or extend the tape.
            match self.alloc_strategy {
                cli::AllocStrategy::TapeIsFixed => {
                    return Err(BfError::DataPtrMovedAfterEnd {
                        program_pointer: self.program_pointer,
                        instruction: self.program.instructions()[self.program_pointer],
                    });
                }
                cli::AllocStrategy::TapeCanGrow => {
                    // Gone past end of tape, but tape can be extended so add another cell
                    self.tape.push(T::default());
                }
            }
        }
        self.data_pointer += 1;
        Ok(())
    }

    /// Moves the data pointer backward
    pub fn move_data_pointer_back(&mut self) -> Result<(), BfError> {
        if self.data_pointer == 0 {
            return Err(BfError::DataPtrMovedBeforeStart {
                program_pointer: self.program_pointer,
                instruction: self.program.instructions()[self.program_pointer],
            });
        }
        self.data_pointer -= 1;
        Ok(())
    }

    // Data value handling methods
    // ###########################

    /// Increment the value of the cell currently pointed to by the data pointer
    pub fn increment_data_value(&mut self) -> Result<(), BfError> {
        self.tape[self.data_pointer] = self.tape[self.data_pointer].inc();
        Ok(())
    }

    /// Decrement the value of the cell currently pointed to by the data pointer
    pub fn decrement_data_value(&mut self) -> Result<(), BfError> {
        self.tape[self.data_pointer] = self.tape[self.data_pointer].dec();
        Ok(())
    }

    /// Get the current value of the cell at the current data pointer position
    // Note: Used for tests
    pub fn get_data_value(&self) -> u8 {
        self.tape[self.data_pointer].to_u8()
    }

    /// Set the current value of the cell at the current data pointer position
    // Note: Used for tests
    pub fn set_data_value(&mut self, value: u8) {
        self.tape[self.data_pointer] = <T as CellKind>::from_u8(value);
    }

    /// Output the value of the cell currently pointed to by the data pointer
    ///
    /// Example usage:
    /// ```
    ///     let program = bft_types::BfProgram::new(&"tiny.bf", "><+-.").unwrap();
    ///     let mut tape: bft_interp::BfTape<u8> =
    ///                         bft_interp::BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed, cli::OutputFormat::BinaryOutput);
    ///     let mut writer = std::io::Cursor::new(Vec::new());
    ///     assert_eq!(tape.command_output_value(&mut writer).is_ok(), true);
    ///     assert_eq!(writer.into_inner()[0], 48);
    /// ```
    pub fn output_value<W: Write>(&mut self, writer: &mut W) -> Result<(), BfError> {
        // Get the value of the cell in the tape at the current data pointer location
        let data = [self.tape[self.data_pointer].to_u8(); 1];

        self.newline = false;

        // Write to where ever it's going, handling any i/o errors.
        // Also
        if self.output_format == OutputFormat::BinaryOutput {
            let mut num = data[0].to_string();
            num += ",";
            writer.write(num.as_bytes()).map_err(|e| BfError::IOError {
                error_msg: e,
                filepath: self.program.filename().to_path_buf(),
                instruction: self.program.instructions()[self.program_pointer],
                program_pointer: self.program_pointer,
            })?;
        } else {
            writer.write(&data).map_err(|e| BfError::IOError {
                error_msg: e,
                filepath: self.program.filename().to_path_buf(),
                instruction: self.program.instructions()[self.program_pointer],
                program_pointer: self.program_pointer,
            })?;
            if data[0] == 0x0a {
                self.newline = true;
            }
        }

        if self.debug() >= cli::DebugLevelType::Verbose {
            println!("Data={:?}", data[0]);
        }

        Ok(())
    }

    /// Input a value into the cell currently pointed to by the data pointer
    ///
    /// Example usage:
    /// ```
    ///     let program = bft_types::BfProgram::new(&"inout.bf", ",.").unwrap();
    ///     let mut tape: bft_interp::BfTape<u8> =
    ///                             bft_interp::BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed, cli::OutputFormat::BinaryOutput);
    ///     let mut reader = std::io::Cursor::new(vec![55]);
    ///     assert!(tape.command_input_value(&mut reader).is_ok());
    /// ```
    pub fn input_value<R: Read>(&mut self, reader: &mut R) -> Result<(), BfError> {
        // Provide a place to put the byte read in. Only one character at a time is read
        let mut data = [0; 1];

        // Read the byte in, handling any i/o errors
        let n = reader.read(&mut data).map_err(|e| BfError::IOError {
            error_msg: e,
            filepath: self.program.filename().to_path_buf(),
            instruction: self.program.instructions()[self.program_pointer],
            program_pointer: self.program_pointer,
        })?;

        if self.debug() >= cli::DebugLevelType::Verbose {
            println!("Data={:?}", data[0]);
        }

        if n == 0 {
            // End of file, use special value of -1 which is how rot13.bf program knows when to terminate
            self.tape[self.data_pointer] = T::from_u8(u8::MAX);
        } else {
            // Place the byte into the tape at the current data pointer location
            self.tape[self.data_pointer] = T::from_u8(data[0]);
        }
        Ok(())
    }

    // Program handling methods
    // ########################

    /// Current program pointer
    pub fn program_pointer(&self) -> usize {
        self.program_pointer
    }

    /// The instruction at the current program pointer
    pub fn current_instruction(&self) -> bft_types::BfInstruction {
        self.program.instructions()[self.program_pointer]
    }

    /// Moves the program pointer forward
    // Note: Used for tests
    pub fn move_program_pointer_forward(&mut self) -> Result<(), BfError> {
        if self.program_pointer == self.program.instructions().len() - 1 {
            return Err(BfError::ProgramPtrMovedAfterEnd {
                program_pointer: self.program_pointer,
                instruction: self.program.instructions()[self.program_pointer],
            });
        }
        self.program_pointer += 1;
        Ok(())
    }

    /// Jump forward to the matching bracket if the value at the current data pointer is zero
    // TODO: Uses a brute force method of finding the matching brackets. Have found a crate
    // that can help called BiMap which should make the matching up easier.
    pub fn jump_forward(&mut self) -> Result<(), BfError> {
        if self.get_data_value() == 0 {
            // Condition satisfied for jump forward, find the matching bracket
            let mut found = false;
            if self.debug() >= cli::DebugLevelType::Verbose {
                println!(
                    "Looking for jump loc at {}",
                    self.current_instruction().location()
                );
            }
            for jmp in self.program.jump_locations() {
                if jmp.forward().line() == self.current_instruction().location().line()
                    && jmp.forward().offset() == self.current_instruction().location().offset()
                {
                    // Matching bracket found, now find it's place in the program
                    // by checking the line and char offset as the program vector
                    // does not link 1-to-1 with the source file.
                    for (i, ins) in self.program.instructions().iter().enumerate() {
                        if ins.location().line() == jmp.backward().line()
                            && ins.location().offset() == jmp.backward().offset()
                        {
                            if self.debug() >= cli::DebugLevelType::Verbose {
                                println!("Jumping to {} at {}", i, ins.location());
                            }
                            self.program_pointer = i; // +1 is added after every instruction
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                // Should never happen since unpaired brackets are checked for before program is run
                return Err(BfError::BracketNotFound {
                    program_pointer: self.program_pointer,
                });
            }
        };
        Ok(())
    }

    /// Jump backward to the matching bracket if the value at the current data pointer is non-zero
    // TODO: Uses a brute force method of finding the matching brackets. Have found a crate
    // that can help called BiMap which should make the matching up easier.
    pub fn jump_backward(&mut self) -> Result<(), BfError> {
        if self.get_data_value() != 0 {
            // Condition satisfied for jump back, find the matching bracket
            let mut found = false;
            if self.debug() >= cli::DebugLevelType::Verbose {
                println!(
                    "Looking for jump loc at {}",
                    self.current_instruction().location()
                );
            }
            for jmp in self.program.jump_locations() {
                if jmp.backward().line() == self.current_instruction().location().line()
                    && jmp.backward().offset() == self.current_instruction().location().offset()
                {
                    // Matching bracket found, now find it's place in the program
                    // by checking the line and char offset as the program vector
                    // does not link 1-to-1 with the source file.
                    for (i, ins) in self.program.instructions().iter().enumerate() {
                        if ins.location().line() == jmp.forward().line()
                            && ins.location().offset() == jmp.forward().offset()
                        {
                            if self.debug() >= cli::DebugLevelType::Verbose {
                                println!("Jumping to {} at {}", i, ins.location());
                            }
                            self.program_pointer = i; // +1 is added after every instruction
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                // Should never happen since unpaired brackets are checked for before program is run
                return Err(BfError::BracketNotFound {
                    program_pointer: self.program_pointer,
                });
            }
        };
        Ok(())
    }

    // BF Command implementation methods
    // #################################

    /// Increment the value in the data cell currently pointed to by the data pointer
    /// The return value is the updated program pointer
    pub fn command_inc_value(&mut self) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("Inc at {}", self.program_pointer());
        }
        self.increment_data_value()?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    /// Decrement the value in the data cell currently pointed to by the data pointer
    pub fn command_dec_value(&mut self) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("Dec at {}", self.program_pointer());
        }
        self.decrement_data_value()?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    /// Move data pointer forward to next data cell in tape
    pub fn command_move_pointer_forward(&mut self) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("IncPtr at {}", self.program_pointer());
        }
        self.move_data_pointer_forward()?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    /// Move data pointer back to previous data cell in tape
    pub fn command_move_pointer_back(&mut self) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("DecPtr at {}", self.program_pointer());
        }
        self.move_data_pointer_back()?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    /// Take input from user and place into the current data cell
    pub fn command_input_value<R: Read>(&mut self, reader: &mut R) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("Input at {}", self.program_pointer());
        }
        self.input_value(reader)?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    /// Take input from user and place into the current data cell
    pub fn command_output_value<W: Write>(&mut self, writer: &mut W) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("Output at {}", self.program_pointer());
        }
        self.output_value(writer)?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    /// Jump forward if the value in the current data cell is zero
    pub fn command_jump_forward(&mut self) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("Jumping forward at {}", self.program_pointer());
        }
        self.jump_forward()?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    /// Jump back if the value in the current data cell is not zero
    pub fn command_jump_backward(&mut self) -> Result<usize, BfError> {
        if self.debug() != cli::DebugLevelType::None {
            println!("Jumping backward at {}", self.program_pointer());
        }
        self.jump_backward()?;
        self.program_pointer += 1;
        Ok(self.program_pointer)
    }

    // Debug handling methods
    // ######################

    /// Debug level currently in use
    pub fn debug(&self) -> cli::DebugLevelType {
        self.debug
    }

    /// Default is no debug output, but it can be enabled to various levels of detail
    pub fn set_debug(&mut self, debug: cli::DebugLevelType) {
        self.debug = debug;
    }
}

/// Implementation of the BF program's tape
///
impl<'a, T: std::fmt::Debug + CellKind + std::clone::Clone + std::default::Default> BfTape<'a, T> {
    /// The interpreter of a Brain Fuck program.
    ///
    /// When input or output is required, the standard in/out objects should be used.
    ///
    /// The return is a Result which if empty when OK, but will contain an error types if there was a problem.
    ///
    /// Example usage:
    /// ```
    ///     let program = bft_types::BfProgram::new(&"tiny.bf", ",+.").unwrap();
    ///     let mut tape: bft_interp::BfTape<u8> =
    ///                             bft_interp::BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed, cli::OutputFormat::BinaryOutput);
    ///     match tape.interpreter(&mut std::io::stdin(), &mut std::io::stdout()) {
    ///         Ok(_) => {}
    ///         Err(e) => println!("Error {}", e),
    ///     }
    /// ```

    // Note: The tape "object" handles the program's execution, not the program "object" which
    // is just a static representation of the program. Sounds like there should be another
    // module which handles the running of the program and the interaction between program
    // and tape.
    pub fn interpreter<R: Read, W: Write>(
        mut self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), BfError> {
        if self.debug > cli::DebugLevelType::None {
            for inst in self.program.instructions() {
                println!("{}", inst);
            }
        }

        // Execute the program.
        while self.program_pointer != self.program.instructions().len() {
            let inst = self.program.instructions()[self.program_pointer];
            let cmd = inst.command();
            self.program_pointer = match cmd {
                bft_types::BfCommand::Comment => todo!(), // Do nothing
                bft_types::BfCommand::IncDataPointer => self.command_move_pointer_forward()?,
                bft_types::BfCommand::DecDataPointer => self.command_move_pointer_back()?,
                bft_types::BfCommand::IncValue => self.command_inc_value()?,
                bft_types::BfCommand::DecValue => self.command_dec_value()?,
                bft_types::BfCommand::OutputValue => self.command_output_value(writer)?,
                bft_types::BfCommand::InputValue => self.command_input_value(reader)?,
                bft_types::BfCommand::JumpForward => self.command_jump_forward()?,
                bft_types::BfCommand::JumpBackward => self.command_jump_backward()?,
            };
        }

        // if !self.newline {
        //     println!(); // To ensure that shell prompt is on new line if no debug used and values were output
        // }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that specifying zero for the size creates a tape of the default size.
    #[test]
    fn new_default_size() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let tape: BfTape<u8> = BfTape::new(
            &program,
            0,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        assert_eq!(tape.data_length(), MAX_TAPE_SIZE);
    }

    /// Test for a valid size of the normal base type.
    #[test]
    fn new_size_of_10000() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let tape: BfTape<u8> = BfTape::new(
            &program,
            10000,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        assert_eq!(tape.data_length(), 10000);
    }

    /// Test that an error is raised when moving the data pointer before the start of the tape
    #[test]
    fn data_pointer_moved_before_start() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        tape.reset_data_pointer();
        // Now move the before the beginning of the tape
        let result = tape.move_data_pointer_back();
        assert!(result.is_err());
    }

    /// Test that an error is raised when moving the data pointer after the end of the tape
    #[test]
    fn data_pointer_moved_after_end() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        tape.reset_data_pointer();
        for i in 0..99 {
            if tape.move_data_pointer_forward().is_err() {
                panic!("The tape should have 100 cells {}", i);
            }
        }
        // Now move past the end of the tape
        let result = tape.move_data_pointer_forward();
        assert!(result.is_err());
    }

    /// Test that the tape is extended when moving the data pointer after the end of the tape
    #[test]
    fn data_pointer_moved_after_end_and_can_grow() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeCanGrow,
            cli::OutputFormat::BinaryOutput,
        );
        tape.reset_data_pointer();
        for i in 0..99 {
            if tape.move_data_pointer_forward().is_err() {
                panic!("The tape should have 100 cells {}", i);
            }
        }
        // Now move past the end of the tape
        let result = tape.move_data_pointer_forward();
        assert!(result.is_ok());

        // Check that the tape is now 1 cell longer
        assert_eq!(tape.data_pointer(), 100);
    }

    /// Test that no error is raised when moving the data pointer normally
    #[test]
    fn data_pointer_moved_normally() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        tape.reset_data_pointer();

        // Move data pointer to end
        for _ in 0..99 {
            let result = tape.move_data_pointer_forward();
            assert!(result.is_ok());
        }
        assert_eq!(tape.data_pointer(), 99);

        // Move data pointer back to beginning
        for _ in 0..99 {
            let result = tape.move_data_pointer_back();
            assert!(result.is_ok());
        }
        assert_eq!(tape.data_pointer(), 0);
    }

    /// Test that the value in a cell is set
    #[test]
    fn set_cell_value() {
        let program = BfProgram::new("tiny.bf", "+-").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        tape.reset_data_pointer();
        tape.set_data_value(55);

        assert_eq!(tape.get_data_value(), 55);
    }

    /// Test that the value in a cell is incremented. Also checks that data value can be read.
    #[test]
    fn increment_cell_value() {
        let program = BfProgram::new("increment.bf", "+").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        tape.reset_data_pointer();
        tape.increment_data_value().unwrap();

        // Check that the initial value of zero has been incremented to one
        assert_eq!(tape.get_data_value(), 1);
    }

    /// Test that the value in a cell is incremented. Also checks that data value can be read.
    #[test]
    fn decrement_cell_value() {
        let program = BfProgram::new("decrement.bf", "-").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );
        tape.reset_data_pointer();
        tape.decrement_data_value().unwrap();

        // Check that the initial value of zero has been decremented and wrapped around to 255 (the max in a u8)
        assert_eq!(tape.get_data_value(), 255);
    }

    /// Test that output works
    #[test]
    fn output_cell_value_as_binary() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );

        // Create a writer as a sink for the value that is being output
        let mut writer = std::io::Cursor::new(Vec::new());

        // Output the value from the current cell in the tape to the writer
        assert!(tape.output_value(&mut writer).is_ok());

        // Check that the value was output correctly, as "0"
        assert_eq!(writer.into_inner()[0], 48);
    }

    /// Test that output as ascii works
    #[test]
    fn output_cell_value_as_ascii() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::AsciiOutput,
        );

        // Create a writer as a sink for the value that is being output
        let mut writer = std::io::Cursor::new(Vec::new());

        // Output the value from the current cell in the tape to the writer
        assert!(tape.output_value(&mut writer).is_ok());

        // Check that the value was output correctly, as 0 (null character)
        assert_eq!(writer.into_inner()[0], 0);
    }

    /// Test that input  works
    #[test]
    fn input_cell_value() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );

        // Create a reader as a source for the value that is being input
        let mut reader = std::io::Cursor::new(vec![55, 11, 22]);

        // Input the value into the current cell in the tape
        tape.reset_data_pointer();
        assert!(tape.input_value(&mut reader).is_ok());
        assert!(tape.move_data_pointer_forward().is_ok());
        assert!(tape.input_value(&mut reader).is_ok());
        assert!(tape.move_data_pointer_forward().is_ok());
        assert!(tape.input_value(&mut reader).is_ok());

        // Check that the values were written
        tape.reset_data_pointer();
        assert_eq!(tape.get_data_value(), 55);
        assert!(tape.move_data_pointer_forward().is_ok());
        assert_eq!(tape.get_data_value(), 11);
        assert!(tape.move_data_pointer_forward().is_ok());
        assert_eq!(tape.get_data_value(), 22);
    }

    /// Test that an error is raised when moving the program pointer past the end of the program
    #[test]
    fn program_pointer_moved_after_end() {
        let program = BfProgram::new("tiny.bf", "><").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(
            &program,
            100,
            cli::AllocStrategy::TapeIsFixed,
            cli::OutputFormat::BinaryOutput,
        );

        // Check that program is only two instructions
        assert_eq!(tape.program.instructions().len(), 2);

        // Move program pointer past end of program
        let result = tape.move_program_pointer_forward();
        assert!(result.is_ok());
        let result = tape.move_program_pointer_forward();
        assert!(result.is_err());
    }
}
