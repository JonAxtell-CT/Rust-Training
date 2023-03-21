use bft_interp::AllocStrategy;
use clap::{arg, value_parser, Command};
use std::path::PathBuf;

pub struct Args {
    /// Name of the program
    program: PathBuf,

    /// Number of cells, must be non-zero. The default is 30,000 if not specified
    cells: u32,

    /// Enable tape to auto-extend from the initial size
    extensible: bft_interp::AllocStrategy,

    /// Debug
    debug: bool,
}

impl Args {
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
                    .value_parser(value_parser!(u32).range(1..)),
            )
            .arg(
                arg!(-e --extensible "Tape can grow")
                    .default_value("false")
                    .required(false),
            )
            .arg(arg!(-d --debug "Debug").required(false))
            .get_matches();

        let program_name = matches.get_one::<String>("program").unwrap();
        println!("program is {:?}", program_name);

        let cells = *matches.get_one::<u32>("cells").unwrap();
        println!("Cells is {:?}", cells);

        let extensible = if *matches.get_one::<bool>("extensible").unwrap() {
            bft_interp::AllocStrategy::TapeCanGrow
        } else {
            bft_interp::AllocStrategy::TapeIsFixed
        };
        println!("Extensible is {:?}", extensible);

        let debug = *matches.get_one::<bool>("debug").unwrap();
        println!("Debug is {:?}", debug);

        Args {
            program: program_name.into(),
            cells,
            extensible,
            debug,
        }
    }

    pub fn program(&self) -> &PathBuf {
        &self.program
    }

    pub fn cell_count(&self) -> usize {
        self.cells.try_into().unwrap()
    }

    pub fn extensible(&self) -> AllocStrategy {
        self.extensible
    }

    pub fn debug(&self) -> bool {
        self.debug
    }
}
