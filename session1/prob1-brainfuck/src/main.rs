use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::{error::Error, path::PathBuf};

fn check_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file_path = PathBuf::from("./");
    file_path.push(file_name);
    let _file_path = File::open(file_path)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // println!("Hello, world!\nYou entered: {:?}", args);

    if args.len() > 1 {
        let filename = &args[1];
        // println!("Filename is {}", filename);
        if check_file(filename).is_err() {
            println!("File not found");
            return Ok(());
        }
        let buf = BufReader::new(File::open(filename)?);
        let mut content: Vec<char> = Vec::new();
        // let mut i = 0;
        for line in buf.lines() {
            match line {
                Ok(r) => {
                    // println!("{} = {}", i, r);
                    let mut chars: Vec<char> = r.chars().collect();
                    content.append(&mut chars);
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
            // i += 1;
        }
        // println!("File content is {:?}", content);

        let valid_chars = "><+-.,[]";
        let mut program = String::new();
        for ch in content {
            if valid_chars.contains(ch) {
                program.push(ch);
            }
        }

        println!("Brainfuck program is \"{}\"", program);
    } else {
        println!("You didn't specify a file");
    }
    Ok(())
}
