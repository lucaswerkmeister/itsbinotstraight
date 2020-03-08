use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;
use rand::seq::IteratorRandom;
use unicode_normalization::UnicodeNormalization;

pub fn biword() -> String {
    let file = File::open("biwords").expect("open biwords");
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.expect("biwords lines"));
    let mut rng = rand::thread_rng();
    lines.choose(&mut rng).expect("biwords empty")
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y'
}

pub fn replacement<R: Rng + ?Sized>(biword: &str, rng: &mut R) -> String {
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
                    vowel = is_vowel(c);
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
    let (straight_replacement, hetero_replacement) = match (b_uppercase, i_uppercase) {
        (false, false) => ("straight", "hetero"),
        (true, false) => ("Straight", "Hetero"),
        (true, true) => ("STRAIGHT", "HETERO"),
        (false, true) => ("sTrAiGhT", "hEtErO"),
    };
    let bi_replacement = if rng.gen_ratio(if vowel { 7 } else { 3 }, 10) { straight_replacement } else { hetero_replacement };
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
    use core::u64;
    use rand::rngs::mock;

    fn static_bool_rng(b: bool) -> impl Rng {
        mock::StepRng::new(if b { 0 } else { u64::MAX }, 0)
    }

    #[test]
    #[ignore]
    fn check_static_bool_rng() {
        assert_eq!(true, static_bool_rng(true).gen_bool(0.5));
        assert_eq!(false, static_bool_rng(false).gen_bool(0.5));
        assert_eq!(true, static_bool_rng(true).gen_ratio(1, 2));
        assert_eq!(false, static_bool_rng(false).gen_ratio(1, 2));
    }

    #[test]
    fn replacement_lower_lower() {
        let mut straight_rng = static_bool_rng(true);
        let mut hetero_rng = static_bool_rng(false);
        assert_eq!(replacement("bi", &mut straight_rng), "straight");
        assert_eq!(replacement("bisexual", &mut hetero_rng), "heterosexual");
        assert_eq!(replacement("-bi", &mut straight_rng), "-straight");
        assert_eq!(replacement("bi-sexual", &mut hetero_rng), "hetero-sexual");
        assert_eq!(replacement("bì", &mut straight_rng), "straight\u{300}");
        assert_eq!(replacement("ḃi", &mut hetero_rng), "hetero");
    }
    #[test]
    fn replacement_upper_lower() {
        let mut straight_rng = static_bool_rng(true);
        let mut hetero_rng = static_bool_rng(false);
        assert_eq!(replacement("Bi", &mut straight_rng), "Straight");
        assert_eq!(replacement("Bisexual", &mut hetero_rng), "Heterosexual");
        assert_eq!(replacement("-Bi", &mut straight_rng), "-Straight");
        assert_eq!(replacement("Bi-Sexual", &mut hetero_rng), "Hetero-Sexual");
        assert_eq!(replacement("Bì", &mut straight_rng), "Straight\u{300}");
        assert_eq!(replacement("Ḃi", &mut hetero_rng), "Hetero");
    }
    #[test]
    fn replacement_lower_upper() {
        let mut straight_rng = static_bool_rng(true);
        let mut hetero_rng = static_bool_rng(false);
        assert_eq!(replacement("bI", &mut straight_rng), "sTrAiGhT");
        assert_eq!(replacement("bIsExUaL", &mut hetero_rng), "hEtErOsExUaL");
        assert_eq!(replacement("-bI", &mut straight_rng), "-sTrAiGhT");
        assert_eq!(replacement("bI-sExUaL", &mut hetero_rng), "hEtErO-sExUaL");
        assert_eq!(replacement("bÌ", &mut straight_rng), "sTrAiGhT\u{300}");
        assert_eq!(replacement("ḃI", &mut hetero_rng), "hEtErO");
    }
    #[test]
    fn replacement_upper_upper() {
        let mut straight_rng = static_bool_rng(true);
        let mut hetero_rng = static_bool_rng(false);
        assert_eq!(replacement("BI", &mut straight_rng), "STRAIGHT");
        assert_eq!(replacement("BISEXUAL", &mut hetero_rng), "HETEROSEXUAL");
        assert_eq!(replacement("-BI", &mut straight_rng), "-STRAIGHT");
        assert_eq!(replacement("BI-SEXUAL", &mut hetero_rng), "HETERO-SEXUAL");
        assert_eq!(replacement("BÌ", &mut straight_rng), "STRAIGHT\u{300}");
        assert_eq!(replacement("ḂI", &mut hetero_rng), "HETERO");
    }

}
