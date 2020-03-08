use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::seq::IteratorRandom;
use unicode_normalization::UnicodeNormalization;

fn biword() -> String {
    let file = File::open("biwords").expect("open biwords");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("biwords lines"));
    let mut rng = rand::thread_rng();
    lines.choose(&mut rng).expect("biwords empty")
}

fn replacement(biword: &str) -> &str {
    // ensure that we can recognize the “Bi” in “Bì…”
    let mut chars = biword.nfd();
    let b_uppercase;
    loop {
        match chars.next() {
            Some(b) => {
                if b == 'b' || b == 'B' {
                    b_uppercase = b == 'B';
                    break;
                }
                // TODO other chars
            },
            None => panic!("not a biword: {}", biword),
        }
    }
    let i_uppercase;
    loop {
        match chars.next() {
            Some(i) => {
                if i == 'i' || i == 'I' {
                    i_uppercase = i == 'I';
                    break;
                }
                // TODO other chars
            },
            None => panic!("not a biword: {}", biword),
        }
    }
    match (b_uppercase, i_uppercase) {
        (false, false) => "straight",
        (true, false) => "Straight",
        (true, true) => "STRAIGHT",
        (false, true) => "sTrAiGhT",
    }
}

fn main() {
    let biword = biword();
    let replacement = replacement(&biword);
    println!("{} → {}", biword, replacement);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_lower_lower() {
        assert_eq!(replacement("bi"), "straight");
        assert_eq!(replacement("bisexual"), "straight");
        assert_eq!(replacement("-bi"), "straight");
        assert_eq!(replacement("bì"), "straight");
        assert_eq!(replacement("ḃi"), "straight");
    }
    #[test]
    fn replacement_upper_lower() {
        assert_eq!(replacement("Bi"), "Straight");
        assert_eq!(replacement("Bisexual"), "Straight");
        assert_eq!(replacement("-Bi"), "Straight");
        assert_eq!(replacement("Bì"), "Straight");
        assert_eq!(replacement("Ḃi"), "Straight");
    }
    #[test]
    fn replacement_lower_upper() {
        assert_eq!(replacement("bI"), "sTrAiGhT");
        assert_eq!(replacement("bIsExUaL"), "sTrAiGhT");
        assert_eq!(replacement("-bI"), "sTrAiGhT");
        assert_eq!(replacement("bÌ"), "sTrAiGhT");
        assert_eq!(replacement("ḃI"), "sTrAiGhT");
    }
    #[test]
    fn replacement_upper_upper() {
        assert_eq!(replacement("BI"), "STRAIGHT");
        assert_eq!(replacement("BISEXUAL"), "STRAIGHT");
        assert_eq!(replacement("-BI"), "STRAIGHT");
        assert_eq!(replacement("BÌ"), "STRAIGHT");
        assert_eq!(replacement("ḂI"), "STRAIGHT");
    }

}
