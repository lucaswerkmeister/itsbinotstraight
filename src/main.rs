use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::seq::IteratorRandom;

fn biword() -> String {
    let file = File::open("biwords").expect("open biwords");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("biwords lines"));
    let mut rng = rand::thread_rng();
    lines.choose(&mut rng).expect("biwords empty")
}

fn replacement(biword: &str) -> &str {
    match &biword[..2] {
        "bi" => "straight",
        "Bi" => "Straight",
        "BI" => "STRAIGHT",
        "bI" => "sTrAiGhT",
        _ => panic!("not a biword: {}", biword),
    }
}

fn main() {
    let biword = biword();
    let replacement = replacement(&biword);
    println!("{} → {}", biword, replacement);
}
