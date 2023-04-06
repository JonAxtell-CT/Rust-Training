use bft_types::BfProgram;
use std::io::{Read, Write};
use thiserror::Error;

const MAX_TAPE_SIZE: usize = 30000;

/// Errors that can be returned by functions that handle running the BF program.
///
#[derive(Error, Debug)]
pub enum BfError {
    /// Error to indicate when the data pointer was moved before the start of the tape
    #[error(
        "Error: Data pointer moved before start of tape at {} {}",
        program_pointer,
        instruction
    )]
    DataPtrMovedBeforeStart {
        instruction: bft_types::BfInstruction,
        program_pointer: usize,
    },
    /// Error to indicate when the data pointer was moved after the end of the tape
    #[error(
        "Error: Data pointer moved after end of tape at {} {}",
        program_pointer,
        instruction
    )]
    DataPtrMovedAfterEnd {
        instruction: bft_types::BfInstruction,
        program_pointer: usize,
    },
    /// Error the occurs when reading/writing using the input/output functionality of the tape
    #[error(
        "Error: I/O error {} at {} {} {}",
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
pub trait CellKind:
    Default
    + Clone
    + Copy
    + TryFrom<u8>
    + TryInto<u8>
    + From<u8>
    + ToString
    + PartialEq
    + num_traits::WrappingAdd
    + num_traits::WrappingSub
{
    /// Increment a data cell's value
    fn inc(&self) -> Self
    where
        Self: std::marker::Sized;
    /// Decrement a data cell's value
    fn dec(&self) -> Self
    where
        Self: std::marker::Sized;

    /// Get the value of a data cell
    fn get(&self) -> u8
    where
        Self: std::marker::Sized;

    /// Set the value of a data cell
    fn set(&mut self, value: u8)
    where
        Self: std::marker::Sized;

    /// Convert the value of a data cell to a u8
    fn to_u8(&self) -> u8
    where
        Self: std::marker::Sized;

    /// Set the value of a data cell from a u8
    fn from_u8(value: u8) -> Self
    where
        Self: std::marker::Sized;
}

/// Implementation of the Trait for the cells using u8
///
impl CellKind for u8 {
    /// Increment a data cell's value
    fn inc(&self) -> Self {
        self.wrapping_add(1)
    }
    /// Decrement a data cell's value
    fn dec(&self) -> Self {
        self.wrapping_sub(1)
    }
    /// Get the value of a data cell
    fn get(&self) -> u8 {
        *self
    }
    /// Set the value of a data cell
    fn set(&mut self, value: u8) {
        *self = value
    }
    /// Convert the value of a data cell to a u8
    fn to_u8(&self) -> u8 {
        *self
    }
    /// Set the value of a data cell from a u8
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
    /// The size of the tape
    tape_size: usize,
    /// The tape itself
    tape: Vec<T>,
    /// Debug flag
    debug: cli::DebugLevelType,
}

/// Implementation of the BF program's tape
///
impl<'a, T: CellKind + std::clone::Clone + std::default::Default> BfTape<'a, T> {
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
    ) -> Self {
        Self {
            program_pointer: 0,
            program,
            data_pointer: 0,
            tape_size,
            alloc_strategy,
            tape: if tape_size == 0 {
                vec![Default::default(); MAX_TAPE_SIZE]
            } else {
                vec![Default::default(); tape_size]
            },
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

    /// Moves the data pointer forward
    pub fn move_data_pointer_forward(&mut self) -> Result<(), BfError> {
        if self.data_pointer == self.tape_size {
            if self.alloc_strategy == cli::AllocStrategy::TapeIsFixed {
                return Err(BfError::DataPtrMovedAfterEnd {
                    program_pointer: self.program_pointer,
                    instruction: self.program.instructions()[self.program_pointer],
                });
            }
            self.tape_size += 1;
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

    /// The instruction at the current program pointer
    pub fn current_instruction(&self) -> &bft_types::BfInstruction {
        &self.program.instructions()[self.program_pointer]
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

    /// Get the value of the cell currently pointed to by the data pointer
    pub fn get_data_value(&mut self) -> T {
        self.tape[self.data_pointer].get().into()
    }

    /// Set the value of the cell currently pointed to by the data pointer
    pub fn set_data_value(&mut self, value: u8) {
        self.tape[self.data_pointer].set(value)
    }

    /// Output the value of the cell currently pointed to by the data pointer
    ///
    /// Example usage:
    /// ```
    ///     let program = bft_types::BfProgram::new(&"tiny.bf", "><+-.").unwrap();
    ///     let mut tape: bft_interp::BfTape<u8> = bft_interp::BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
    ///     let mut writer = std::io::Cursor::new(Vec::new());
    ///     assert_eq!(tape.output_value(&mut writer).is_ok(), true);
    ///     assert_eq!(writer.into_inner()[0], 0);
    /// ```
    pub fn output_value<W: Write>(&mut self, writer: &mut W) -> Result<(), BfError> {
        // Get the value of the cell in the tape at the current data pointer location
        let data = [self.tape[self.data_pointer].to_u8(); 1];

        // Write to where ever it's going, handling any i/o errors
        writer.write(&data).map_err(|e| BfError::IOError {
            error_msg: e,
            filepath: self.program.filename().to_path_buf(),
            instruction: self.program.instructions()[self.program_pointer],
            program_pointer: self.program_pointer,
        })?;

        if self.debug() != cli::DebugLevelType::None {
            println!("Data={}", data[0]);
        }

        Ok(())
    }

    /// Input a value into the cell currently pointed to by the data pointer
    ///
    /// Example usage:
    /// ```
    ///     let program = bft_types::BfProgram::new(&"tiny.bf", "><+-.").unwrap();
    ///     let mut tape: bft_interp::BfTape<u8> = bft_interp::BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
    ///     let mut reader = std::io::Cursor::new(vec![55]);
    ///     assert_eq!(tape.input_value(&mut reader).is_ok(), true);
    ///     assert_eq!(tape.get_data_value(), 55);
    /// ```
    pub fn input_value<R: Read>(&mut self, reader: &mut R) -> Result<(), BfError> {
        // Provide a place to put the byte read in
        let mut data = [0; 1];

        // Read the byte in, handling any i/o errors
        reader.read(&mut data).map_err(|e| BfError::IOError {
            error_msg: e,
            filepath: self.program.filename().to_path_buf(),
            instruction: self.program.instructions()[self.program_pointer],
            program_pointer: self.program_pointer,
        })?;

        // Place the byte into the tape at the current data pointer location
        self.set_data_value(data[0]);
        Ok(())
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
    /// The basis of an interpreter for the program

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

        // Note: This doesn't work properly at the moment since the jump instructions
        // haven't been implemented.
        while self.program_pointer != self.program.instructions().len() {
            let inst = self.program.instructions()[self.program_pointer];
            let cmd = inst.command();
            match cmd {
                bft_types::BfCommand::Comment => {}
                bft_types::BfCommand::IncDataPointer => {
                    self.move_data_pointer_forward()?;
                    self.program_pointer += 1;
                }
                bft_types::BfCommand::DecDataPointer => {
                    self.move_data_pointer_back()?;
                    self.program_pointer += 1;
                }
                bft_types::BfCommand::IncValue => {
                    self.increment_data_value()?;
                    self.program_pointer += 1;
                }
                bft_types::BfCommand::DecValue => {
                    self.decrement_data_value()?;
                    self.program_pointer += 1;
                }
                bft_types::BfCommand::OutputValue => {
                    self.output_value(writer)?;
                    self.program_pointer += 1;
                }
                bft_types::BfCommand::InputValue => {
                    self.input_value(reader)?;
                    self.program_pointer += 1;
                }
                bft_types::BfCommand::JumpForward => {}
                bft_types::BfCommand::JumpBackward => {}
            };
        }
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
        let tape: BfTape<u8> = BfTape::new(&program, 0, cli::AllocStrategy::TapeIsFixed);
        assert_eq!(tape.tape.capacity(), MAX_TAPE_SIZE);
    }

    /// Test for a valid size of the normal base type.
    #[test]
    fn new_size_of_10000() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let _tape: BfTape<u8> = BfTape::new(&program, 10000, cli::AllocStrategy::TapeIsFixed);
    }

    /// Test that an error is raised when moving the data pointer before the start of the tape
    #[test]
    fn data_pointer_moved_before_start() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        // Now move the before the beginning of the tape
        let result = tape.move_data_pointer_back();
        assert!(result.is_err());
    }

    /// Test that an error is raised when moving the data pointer after the end of the tape
    #[test]
    fn data_pointer_moved_after_end() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        for i in 0..100 {
            if tape.move_data_pointer_forward().is_err() {
                panic!("The tape should have 100 cells {}", i);
            }
        }
        // Now move past the end of the tape
        let result = tape.move_data_pointer_forward();
        assert!(result.is_err());
    }

    /// Test that the value in a cell is incremented. Also checks that data value can be read.
    #[test]
    fn increment_cell_value() {
        let program = BfProgram::new("increment.bf", "+").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        tape.increment_data_value().unwrap();

        // Check that the initial value of zero has been incremented to one
        assert_eq!(tape.get_data_value(), 1);
    }

    /// Test that the value in a cell is incremented. Also checks that data value can be read.
    #[test]
    fn decrement_cell_value() {
        let program = BfProgram::new("decrement.bf", "-").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        tape.decrement_data_value().unwrap();

        // Check that the initial value of zero has been decremented and wrapped around to 255 (the max in a u8)
        assert_eq!(tape.get_data_value(), 255);
    }

    /// Test that output works
    #[test]
    fn output_cell_value() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        let mut writer = std::io::Cursor::new(Vec::new());
        assert!(tape.output_value(&mut writer).is_ok());
        assert_eq!(writer.into_inner()[0], 0);
    }

    /// Test that input  works
    #[test]
    fn input_cell_value() {
        let program = BfProgram::new("tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        let mut reader = std::io::Cursor::new(vec![55]);
        assert!(tape.input_value(&mut reader).is_ok());
        assert_eq!(tape.get_data_value(), 55);
    }
}
