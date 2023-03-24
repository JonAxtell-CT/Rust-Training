use bft_types::BfProgram;

const MAX_TAPE_SIZE: usize = 30000;

/// A tape is a representation of a Brain Fuck program's data as it's being interpreted.
///
pub struct BfTape<T> {
    /// The data pointer. This is not the instruction pointer.
    #[allow(unused)]
    data_pointer: usize,
    /// Indicates if more memory can be allocated from it's initial size or if it is fixed
    #[allow(unused)]
    grow: cli::AllocStrategy,
    /// The tape itself
    #[allow(unused)]
    cells: Vec<T>,
    /// Debug
    #[allow(unused)]
    debug: cli::DebugLevelType,
}

impl<T> BfTape<T> {
    /// Create a new tape for BF instructions.
    ///
    /// If the size is specified as zero, then the default size of 30,000 cells will be allocated.
    ///
    /// The allocation strategy can be set so that the tape can grow as needed or it can be fixed.
    // Standard method of new
    pub fn new(size: usize, grow: cli::AllocStrategy) -> Self {
        Self {
            data_pointer: 0,
            grow,
            cells: if size == 0 {
                Vec::<T>::with_capacity(MAX_TAPE_SIZE)
            } else {
                Vec::<T>::with_capacity(size)
            },
            debug: cli::DebugLevelType::None,
        }
    }

    pub fn debug(&self) -> cli::DebugLevelType {
        self.debug
    }
    pub fn set_debug(&mut self, debug: cli::DebugLevelType) {
        self.debug = debug;
    }
}

impl<T: std::fmt::Debug> BfTape<T> {
    /// The basis of an interpreter for the program
    pub fn interpreter(self, program: &BfProgram) {
        if self.debug > cli::DebugLevelType::None {
            for inst in program.instructions() {
                println!("{}", inst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that specifying zero for the size creates a tape of the default size.
    #[test]
    fn test_new_zero() {
        let tape: BfTape<u8> = BfTape::new(0, cli::AllocStrategy::TapeIsFixed);
        assert_eq!(tape.cells.capacity(), MAX_TAPE_SIZE);
    }

    // Test for a valid size of the normal base type.
    #[test]
    fn test_new_ok() {
        let _tape: BfTape<u8> = BfTape::new(10000, cli::AllocStrategy::TapeIsFixed);
    }

    // Test for a tape with a different base type.
    #[test]
    fn test_new_ok_u16() {
        let _tape: BfTape<u16> = BfTape::new(10000, cli::AllocStrategy::TapeIsFixed);
    }

    // Test that the maximum size tape isn't exceeded.
    #[test]
    fn test_excessive() {
        let _tape: BfTape<u8> = BfTape::new(50000, cli::AllocStrategy::TapeIsFixed);
    }
}
