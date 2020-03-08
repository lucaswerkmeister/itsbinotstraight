use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::seq::IteratorRandom;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("biwords").expect("open biwords");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("biwords lines"));
    let mut rng = rand::thread_rng();
    println!("{}", lines.choose(&mut rng).expect("biwords empty"));
    Ok(())
}
