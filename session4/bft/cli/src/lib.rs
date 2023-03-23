use clap::{arg, Command};
use std::path::PathBuf;

/// Allocation strategy for the tape that consists of the BF program's data.
///
/// Enumerated values to indicate whether the tape can grow or not instead
/// of an anonymous boolean.
///
/// * TapeCanGrow allows for allocation of more memory when required.
/// * TapeIsFixed doesn't allow the amount of memory used to store the tape to be reallocated.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AllocStrategy {
    /// Allows more memory to be allocated when required.
    TapeCanGrow,
    /// The amount of memory used to store the tape cannot be changed.
    TapeIsFixed,
}

/// Debug levels
///
/// Enumerated levels to indicate the verbosity of the debug output rather
/// then use magic numbers. Also easier to read and understand.
///
/// * None
/// * Information
/// * Verbose
/// * Detailed
///
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DebugLevelType {
    /// No debug output
    None,
    /// Only output informational debug stuff
    Information,
    /// More information and debug data
    Verbose,
    /// Very detailed and very verbose amount of debug information
    Detailed,
}

impl std::convert::From<u8> for DebugLevelType {
    /// Convert a u8 (typically from a CLI argument) into the appropriate
    /// debug enum.
    fn from(orig: u8) -> Self {
        match orig {
            0 => return DebugLevelType::None,
            1 => return DebugLevelType::Information,
            2 => return DebugLevelType::Verbose,
            3 => return DebugLevelType::Detailed,
            _ => return DebugLevelType::None,
        }
    }
}

pub struct Args {
    /// Name of the program
    program: PathBuf,

    /// Number of cells, must be non-zero. The default is 30,000 if not specified
    cells: usize,

    /// Enable tape to auto-extend from the initial size
    extensible: AllocStrategy,

    /// Debug
    debug: u8,
}

impl Args {
    /// Create a new instance of the arguments to the program
    pub fn new() -> Self {
        let matches = Command::new("bft")
            .version("1.0")
            .author("J Axtell <jonaxtell@codethink.co.uk>")
            .about("Runs a BF program")
            .arg(arg!(<program> "Name of BF program").required(true))
            .arg(
                arg!(cells: -c --cells <count> "Number of initial cells in the tape")
                    .default_value("30000")
                    .required(false)
                    .value_parser(clap::value_parser!(u32).range(1..)),
            )
            .arg(
                arg!(-e --extensible "Tape can grow")
                    .default_value("false")
                    .required(false),
            )
            .arg(
                arg!(-d --debug "Debug")
                    .required(false)
                    .action(clap::ArgAction::Count),
            )
            .get_matches();

        let program_name = matches.get_one::<String>("program").unwrap();
        println!("program is {:?}", program_name);

        let cells = matches.get_one::<u32>("cells").unwrap();
        println!("Cells is {:?}", cells);

        let extensible = if *matches.get_one::<bool>("extensible").unwrap() {
            AllocStrategy::TapeCanGrow
        } else {
            AllocStrategy::TapeIsFixed
        };
        println!("Extensible is {:?}", extensible);

        let debug = matches.get_count("debug").into();
        println!("Debug is {:?}", debug);

        Args {
            program: program_name.into(),
            cells: *cells as usize,
            extensible,
            debug,
        }
    }

    /// Name of BF source file
    pub fn program(&self) -> &PathBuf {
        &self.program
    }

    /// The number of cells in the BF program's tape. The number of cells cannot be zero.
    pub fn cell_count(&self) -> usize {
        self.cells
    }

    /// Flag indicating if the BF program's tape can grow or is fixed
    /// * TapeCanGrow allows for allocation of more memory when required.
    /// * TapeIsFixed doesn't allow the amount of memory used to store the tape to be reallocated.
    pub fn extensible(&self) -> AllocStrategy {
        self.extensible
    }

    /// Flag indicating the amount of debug to output
    /// * None
    /// * Information
    /// * Verbose
    /// * Detailed
    pub fn debug(&self) -> DebugLevelType {
        self.debug.into()
    }
}
