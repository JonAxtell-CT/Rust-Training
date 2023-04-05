use bft_types::BfProgram;
use thiserror::Error;

const MAX_TAPE_SIZE: usize = 30000;

/// Errors that can be returned by functions that handle running the BF program.
///
#[derive(Error, Debug, PartialEq)]
pub enum BfError {
    #[error(
        "Error: Data pointer moved before start of tape at {}",
        program_pointer
    )]
    DataPtrMovedBeforeStart { program_pointer: usize },
    #[error("Error: Data pointer moved after end of tape at {}", program_pointer)]
    DataPtrMovedAfterEnd { program_pointer: usize },
}

/// Trait for the cells in the tape that allows them to be incremented/decremented.
///
pub trait CellKind {
    /// Increment a data cell's value
    fn inc(&self) -> Self
    where
        Self: std::marker::Sized;
    /// Decrement a data cell's value
    fn dec(&self) -> Self
    where
        Self: std::marker::Sized;
}

impl<T: num_traits::WrappingAdd + num_traits::WrappingSub + From<u8>> CellKind for T {
    /// Increment a data cell's value
    fn inc(&self) -> Self {
        self.wrapping_add(&T::from(1))
    }
    /// Decrement a data cell's value
    fn dec(&self) -> Self {
        self.wrapping_sub(&T::from(1))
    }
}
/// A tape is a representation of a Brain Fuck program's data as it's being interpreted.
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
    /// Debug
    debug: cli::DebugLevelType,
}

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
            });
        }
        self.data_pointer -= 1;
        Ok(())
    }

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
    pub fn get_data_value(&mut self) -> &T {
        &self.tape[self.data_pointer]
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

impl<'a, T: std::fmt::Debug> BfTape<'a, T> {
    /// The basis of an interpreter for the program
    pub fn interpreter(self) {
        if self.debug > cli::DebugLevelType::None {
            for inst in self.program.instructions() {
                println!("{}", inst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that specifying zero for the size creates a tape of the default size.
    #[test]
    fn new_default_size() {
        let program = BfProgram::new(&"tiny.bf", "><+-.").unwrap();
        let tape: BfTape<u8> = BfTape::new(&program, 0, cli::AllocStrategy::TapeIsFixed);
        assert_eq!(tape.tape.capacity(), MAX_TAPE_SIZE);
    }

    /// Test for a valid size of the normal base type.
    #[test]
    fn new_ok_u8() {
        let program = BfProgram::new(&"tiny.bf", "><+-.").unwrap();
        let _tape: BfTape<u8> = BfTape::new(&program, 10000, cli::AllocStrategy::TapeIsFixed);
    }

    /// Test for a tape with a different base type.
    #[test]
    fn new_ok_u16() {
        let program = BfProgram::new(&"tiny.bf", "><+-.").unwrap();
        let _tape: BfTape<u16> = BfTape::new(&program, 10000, cli::AllocStrategy::TapeIsFixed);
    }

    /// Test that the maximum size tape isn't exceeded.
    #[test]
    fn new_excessive() {
        let program = BfProgram::new(&"tiny.bf", "><+-.").unwrap();
        let _tape: BfTape<u8> = BfTape::new(&program, 50000, cli::AllocStrategy::TapeIsFixed);
    }

    /// Test that an error is raised when moving the data pointer before the start of the tape
    #[test]
    fn data_pointer_moved_before_start() {
        let program = BfProgram::new(&"tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        assert_eq!(
            tape.move_data_pointer_back(),
            Err(BfError::DataPtrMovedBeforeStart { program_pointer: 0 })
        );
    }

    /// Test that an error is raised when moving the data pointer after the end of the tape
    #[test]
    fn data_pointer_moved_after_end() {
        let program = BfProgram::new(&"tiny.bf", "><+-.").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        for i in 0..100 {
            if tape.move_data_pointer_forward().is_err() {
                panic!("The tape should have 100 cells {}", i);
            }
        }
        // Now move past the end of the tape
        assert_eq!(
            tape.move_data_pointer_forward(),
            Err(BfError::DataPtrMovedAfterEnd { program_pointer: 0 })
        );
    }

    /// Test that the value in a cell is incremented
    #[test]
    fn increment_cell_value() {
        let program = BfProgram::new(&"increment.bf", "+").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        let _ans = tape.increment_data_value().unwrap();

        // Check that the initial value of zero has been incremented to one
        assert_eq!(*tape.get_data_value(), 1);
    }

    /// Test that the value in a cell is incremented
    #[test]
    fn decrement_cell_value() {
        let program = BfProgram::new(&"increment.bf", "+").unwrap();
        let mut tape: BfTape<u8> = BfTape::new(&program, 100, cli::AllocStrategy::TapeIsFixed);
        tape.reset_data_pointer();
        let _ans = tape.decrement_data_value().unwrap();

        // Check that the initial value of zero has been decremented and wrapped around to 255 (the max in a u8)
        assert_eq!(*tape.get_data_value(), 255);
    }
}
