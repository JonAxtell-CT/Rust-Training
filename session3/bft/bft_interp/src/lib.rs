use bft_types::BfProgram;

#[allow(dead_code)]
const DEBUG: bool = false; // Set to true to enable debugging code

const MAX_TAPE_SIZE: usize = 30000;

/// Allocation strategy for the tape that consists of the BF program.
///
/// * TapeCanGrow allows for allocation of more memory when required.
/// * TapeIsFixed doesn't allow the amount of memory used to store the tape to be reallocated.
///
pub enum AllocStrategy {
    /// Allows more memory to be allocated when required.
    TapeCanGrow,
    /// The amount of memory used to store the tape cannot be changed.
    TapeIsFixed,
}

pub struct BfTape<T> {
    /// The data pointer. This is not the instruction pointer.
    #[allow(unused)]
    data_pointer: usize,
    /// Indicates if more memory can be allocated from it's initial size or if it is fixed
    #[allow(unused)]
    grow: AllocStrategy,
    /// The tape itself
    #[allow(unused)]
    cells: Vec<T>,
}

/// A tape is a representation of a Brain Fuck program as it's being interpreted.
///
impl<T> BfTape<T> {
    // /// Create a new tape for BF instructions.
    // ///
    // /// If the size is specified as zero, then the default size of 30,000 cells will be allocated.
    // /// The maximum size of the tape is 30,000.
    // ///
    // /// The allocation strategy can be set so that the tape can grow as needed or it can be fixed.
    // // Standard method of new
    // pub fn new(size: usize, grow: bool) -> Self {
    //     Self {
    //         data_pointer: 0,
    //         grow: grow,
    //         cells: if size == 0 {
    //             Vec::<T>::with_capacity(MAX_TAPE_SIZE)
    //         } else if size <= MAX_TAPE_SIZE {
    //             Vec::<T>::with_capacity(size)
    //         } else {
    //             // Panic rather than return error since new should never fail
    //             panic!("Tape can be a maximum of {MAX_TAPE_SIZE}, not {size}")
    //         },
    //     }
    // }

    /// Create a new tape for BF instructions.
    ///
    /// If the size is specified as zero, then the default size of 30,000 cells will be allocated.
    /// The maximum size of the tape is 30,000.
    ///
    /// The allocation strategy can be set so that the tape can grow as needed or it can be fixed.
    // Spicy method of new
    pub fn new(size: usize, grow: AllocStrategy) -> Self {
        // let nsize = core::num::NonZeroUsize::new(size);
        Self {
            data_pointer: 0,
            grow,
            cells: match core::num::NonZeroUsize::new(size) {
                None => Vec::<T>::with_capacity(MAX_TAPE_SIZE),
                Some(s) => {
                    if size <= MAX_TAPE_SIZE {
                        Vec::<T>::with_capacity(s.into())
                    } else {
                        // Panic rather than return error since new should never fail
                        panic!("Tape can be a maximum of {MAX_TAPE_SIZE}, not {size}")
                    }
                }
            },
        }
    }

    pub fn capacity(self) -> usize {
        self.cells.capacity()
    }
}

impl<T: std::fmt::Debug> BfTape<T> {
    /// The basis of an interpreter for the program
    pub fn interpreter(self, program: &BfProgram) {
        for inst in program.instructions() {
            println!("{:?}", inst);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that specifying zero for the size creates a tape of the default size.
    #[test]
    fn test_new_zero() {
        let tape: BfTape<u8> = BfTape::new(0, AllocStrategy::TapeIsFixed);
        assert_eq!(tape.capacity(), MAX_TAPE_SIZE);
    }

    // Test for a valid size of the normal base type.
    #[test]
    fn test_new_ok() {
        let _tape: BfTape<u8> = BfTape::new(10000, AllocStrategy::TapeIsFixed);
    }

    // Test for a tape with a different base type.
    #[test]
    fn test_new_ok_u16() {
        let _tape: BfTape<u16> = BfTape::new(10000, AllocStrategy::TapeIsFixed);
    }

    // Test that the maximum size tape isn't exceeded.
    #[test]
    #[should_panic(expected = "Tape can be a maximum of 30000, not 50000")]
    fn test_excessive() {
        let _tape: BfTape<u8> = BfTape::new(50000, AllocStrategy::TapeIsFixed);
    }
}
