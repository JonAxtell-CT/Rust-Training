use clap::{arg, value_parser, Command};
use std::path::PathBuf;

pub struct Args {
    /// Name of the program
    pub program: PathBuf,

    /// Number of cells, must be non-zero. The default is 30,000 if not specified
    pub cells: i32,

    /// Enable tape to auto-extend from the initial size
    pub extensible: bool,

    /// Debug
    pub debug: bool,
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
                    .value_parser(value_parser!(i32).range(1..)),
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

        let cell_count = matches
            .get_one::<i32>("cells")
            .expect("Cells must be nonzero");
        println!("Cells is {:?}", cell_count);

        let extensible = matches.get_one::<bool>("extensible").unwrap();
        println!("Extensible is {:?}", extensible);

        let debug = matches.get_one::<bool>("debug").unwrap();
        println!("Debug is {:?}", debug);

        Args {
            program: program_name.into(),
            cells: *cell_count,
            extensible: *extensible,
            debug: *debug,
        }
    }
}
