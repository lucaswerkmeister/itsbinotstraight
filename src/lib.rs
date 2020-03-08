use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::seq::IteratorRandom;
use unicode_normalization::UnicodeNormalization;

pub fn biword() -> String {
    let file = File::open("biwords").expect("open biwords");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("biwords lines"));
    let mut rng = rand::thread_rng();
    lines.choose(&mut rng).expect("biwords empty")
}

pub fn replacement(biword: &str) -> String {
    // ensure that we can recognize the “Bi” in “Bì…”
    let mut chars = biword.nfd();
    // characters that occur before the “bi” (“-biotic” is a valid biword)
    let mut before_b = String::new();
    // whether the “b” is uppercase or not
    let b_uppercase;
    loop {
        match chars.next() {
            Some(b) => {
                if b == 'b' || b == 'B' {
                    b_uppercase = b == 'B';
                    break;
                } else {
                    before_b.push(b);
                }
            },
            None => panic!("not a biword: {}", biword),
        }
    }
    // whether the “i” is uppercase or not
    let i_uppercase;
    loop {
        match chars.next() {
            Some(i) => {
                if i == 'i' || i == 'I' {
                    i_uppercase = i == 'I';
                    break;
                } else if i.is_alphanumeric() {
                    panic!("not a biword: {}", biword);
                } else {
                    // could be a diacritic or something else; drop
                }
            },
            None => panic!("not a biword: {}", biword),
        }
    }
    // characters that occur after the “bi” (only as far as we’ve consumed chars)
    let mut after_i = String::new();
    // whether “bi” is followed by a vowel (prefer “straight”) or not (prefer “hetero”)
    let vowel;
    loop {
        match chars.next() {
            Some(c) => {
                if c.is_alphanumeric() {
                    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y' {
                        vowel = false;
                    } else {
                        vowel = true;
                    }
                    after_i.push(c);
                    break;
                } else {
                    // could be a diacritic or something else (“bi-weekly” is a valid biword);
                    // no stdlib way to distinguish AFAIK, keep for now
                    after_i.push(c);
                }
            },
            None => {
                vowel = false;
                break;
            }
        }
    }
    let bi_replacement = match (b_uppercase, i_uppercase) {
        (false, false) => "straight",
        (true, false) => "Straight",
        (true, true) => "STRAIGHT",
        (false, true) => "sTrAiGhT",
    };
    let mut ret = String::with_capacity(biword.len() + ("straight".len() - "bi".len()));
    ret.push_str(&before_b);
    ret.push_str(&bi_replacement);
    ret.push_str(&after_i);
    ret.extend(chars);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_lower_lower() {
        assert_eq!(replacement("bi"), "straight");
        assert_eq!(replacement("bisexual"), "straightsexual");
        assert_eq!(replacement("-bi"), "-straight");
        assert_eq!(replacement("bi-sexual"), "straight-sexual");
        assert_eq!(replacement("bì"), "straight\u{300}");
        assert_eq!(replacement("ḃi"), "straight");
    }
    #[test]
    fn replacement_upper_lower() {
        assert_eq!(replacement("Bi"), "Straight");
        assert_eq!(replacement("Bisexual"), "Straightsexual");
        assert_eq!(replacement("-Bi"), "-Straight");
        assert_eq!(replacement("Bi-Sexual"), "Straight-Sexual");
        assert_eq!(replacement("Bì"), "Straight\u{300}");
        assert_eq!(replacement("Ḃi"), "Straight");
    }
    #[test]
    fn replacement_lower_upper() {
        assert_eq!(replacement("bI"), "sTrAiGhT");
        assert_eq!(replacement("bIsExUaL"), "sTrAiGhTsExUaL");
        assert_eq!(replacement("-bI"), "-sTrAiGhT");
        assert_eq!(replacement("bI-sExUaL"), "sTrAiGhT-sExUaL");
        assert_eq!(replacement("bÌ"), "sTrAiGhT\u{300}");
        assert_eq!(replacement("ḃI"), "sTrAiGhT");
    }
    #[test]
    fn replacement_upper_upper() {
        assert_eq!(replacement("BI"), "STRAIGHT");
        assert_eq!(replacement("BISEXUAL"), "STRAIGHTSEXUAL");
        assert_eq!(replacement("-BI"), "-STRAIGHT");
        assert_eq!(replacement("BI-SEXUAL"), "STRAIGHT-SEXUAL");
        assert_eq!(replacement("BÌ"), "STRAIGHT\u{300}");
        assert_eq!(replacement("ḂI"), "STRAIGHT");
    }

}
