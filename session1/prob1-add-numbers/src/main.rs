use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!\nYou entered: {:?}", args);

    if args.len() > 1 {
        let filename = &args[1];
        // println!("Filename is {}", filename);
        let fd = File::open(filename)?;
        let buf = BufReader::new(fd);
        let mut i = 0;
        let mut total = 0;
        for line in buf.lines() {
            match line {
                Ok(r) => {
                    let num: i32 = r
                        .trim()
                        .parse()
                        .expect("File can only contain integers, one per line is invalid");
                    // println!("{} = {}", i, num);
                    total += num
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
            i += 1;
        }
        println!("Total is {}", total);
    } else {
        println!("You didn't specify a file");
    }
    Ok(())
}
