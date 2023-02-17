use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/*
 * Name and score line
 * Handles lines in score file that contain a name with an associated score
 */
#[derive(Debug)]
struct NameNumberLine {
    name: String,
    score: u32,
}

impl NameNumberLine {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
}

impl fmt::Display for NameNumberLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.score)
    }
}

/*
 * Name only line
 * Handles lines in score file that only have a name with no score
 */
#[derive(Debug)]
struct NameOnlyLine {
    name: String,
}

impl NameOnlyLine {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl fmt::Display for NameOnlyLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/*
 * Lines
 * Each entry is a name and score or just a name
 */
#[derive(Debug)]
enum Lines {
    NN(NameNumberLine),
    NO(NameOnlyLine),
}

impl TryFrom<&String> for Lines {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &String) -> Result<Self, Box<dyn std::error::Error>> {
        if value.contains(":") {
            let value = value.split_once(":").ok_or("Huh?")?;
            if value.0.is_empty() {
                return Err("No name".into());
            }
            if value.1.is_empty() {
                return Err("No score".into());
            }
            // println!("{:?}", v);
            let n = value.0;
            let s = value.1.parse::<u32>()?;
            Ok(Lines::NN(NameNumberLine {
                name: n.to_string(),
                score: s,
            }))
        } else {
            // println!("{:?}", value);
            Ok(Lines::NO(NameOnlyLine {
                name: value.to_string(),
            }))
        }
    }
}

impl fmt::Display for Lines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Lines::NN(n_a_n) => write!(f, "{}", n_a_n),
            Lines::NO(no) => write!(f, "{}", no),
        }
    }
}

/*
 * Scores
 * Analysis of the scores on the doors
 */
#[derive(Debug)]
struct Scores {
    total_score: u32,
    tests_taken: u32,
    missed_tests: u32,
}

impl Scores {
    fn add_score(&mut self, score: u32) {
        self.total_score += score;
        self.tests_taken += 1;
    }
    fn missed_test(&mut self) {
        self.missed_tests += 1;
    }
    fn get_total_score(&self) -> u32 {
        self.total_score
    }
    fn get_tests_taken(&self) -> u32 {
        self.tests_taken
    }
    fn get_missed_tests(&self) -> u32 {
        self.missed_tests
    }
}

impl std::default::Default for Scores {
    fn default() -> Self {
        Scores {
            total_score: 0,
            tests_taken: 0,
            missed_tests: 0,
        }
    }
}

fn pluralize<T: num::Integer>(value: T) -> String {
    if value.is_one() {
        "".to_string()
    } else {
        "s".to_string()
    }
}
/*
 * Pretty printing of the scores on the doors.
 * To be used in output such as println!("{} took {}", name, score)
 * and will output something along the lines of
 * "n tests, with a total score of y, and they missed z tests"
 * with the missed tests only output if they actually missed a test
 */
impl fmt::Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} test{}, with a total score of {}",
            self.get_tests_taken(),
            pluralize(self.get_tests_taken()),
            self.get_total_score()
        )?;
        if self.get_missed_tests() > 0 {
            write!(
                f,
                ", and they missed {} test{}",
                self.get_missed_tests(),
                pluralize(self.get_missed_tests()),
            )?;
        };
        Ok(())
    }
}

/*
 * Parse the scores file and collect all the lines into one of two different
 * structures within a enum and return a vector containing everything
 */
fn parse_score_file(filename: String) -> Result<Vec<Lines>, Box<dyn std::error::Error>> {
    // println!("Filename is {}", filename);

    let fd = File::open(&filename).map_err(|e| format!("Could not open file '{filename}': {e}"))?;
    let buf = BufReader::new(fd);
    let mut lines = Vec::<Lines>::new();
    let mut i = 1;
    for line in buf.lines() {
        let line =
            line.map_err(|e| format!("Problem reading from file '{filename}' at line {i}': {e}"))?;
        let e = Lines::try_from(&line)?;
        lines.push(e);
        i += 1;
    }
    Ok(lines)
}

/*
 * Main
 * Read a file containing names and scores. Each name is followed by a colon
 * and a score. Or alternatively is just a name without a colon or score. Names
 * can occur multiple times. A name without a score means a missed test and is
 * not counted as a zero score.
 */
fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().nth(1).ok_or("You didn't specify a file")?;

    // Parse the text file into structures in memory
    let lines = parse_score_file(filename.to_string())?;

    // Dump out the result of the processing of the text file
    if cfg!(debug_assertions) {
        println!("Lines from the score file");
        for line in lines.iter() {
            println!("{:?}", line);
        }
        println!("Score file as an iterator");
        println!("{:?}", lines.iter()); // Debug output (concise)
        println!("{:#?}", lines.iter()); // Pretty debug output (verbose)
        println!("Count: {}", lines.len());
    }

    // Do some processing of the text file to get some totals
    let mut scores = Scores::default();
    for line in lines.iter() {
        match line {
            Lines::NN(nn) => {
                scores.add_score(nn.score);
            }
            Lines::NO(_no) => {
                scores.missed_test();
            }
        }
    }
    println!(
        "Scores: Total={}, Count={}, Missed={}",
        scores.get_total_score(),
        scores.get_tests_taken(),
        scores.get_missed_tests()
    );

    // Build a hash map of the people in the scores file to get some stats, handling
    // those with no score who missed tests and those with scores
    let mut hm: HashMap<String, Scores> = HashMap::new();
    for line in lines.iter() {
        match line {
            Lines::NN(n_a_n) => {
                hm.entry(n_a_n.get_name().to_string())
                    .or_default()
                    .add_score(n_a_n.get_score());
            }
            Lines::NO(no) => {
                hm.entry(no.get_name().to_string())
                    .or_default()
                    .missed_test();
            }
        }
    }

    #[cfg(debug_assertions)]
    println!("Hash={:?}", hm);

    // Output the information gathered from the scores file in a nice human readable manner
    for (n, s) in hm.iter() {
        println!("{} took {}", n, s);
    }
    Ok(())
}
