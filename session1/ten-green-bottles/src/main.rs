use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let verses = env::args().nth(1).ok_or("How many verses should I sing?")?;
    let verses = verses.parse()?;

    for verse in (1..=verses).rev() {
        sing_verse(verse);
    }
    Ok(())
}

// Do not worry about the 'static here, consider it "magic" for now
fn bottles(n: u8) -> &'static str {
    if n == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

fn sing_verse(vnum: u8) {
    println!("{} green {} hanging on the wall", vnum, bottles(vnum));
    println!("{} green {} hanging on the wall", vnum, bottles(vnum));
    println!("And if one green bottle should accidentally fall");
    if vnum > 1 {
        println!(
            "There'll be {} green {} hanging on the wall",
            vnum - 1,
            bottles(vnum - 1)
        );
    } else {
        println!("There'll be no green bottles hanging on the wall");
    }
    println!();
}
