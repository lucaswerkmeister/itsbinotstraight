fn main() {
    let biword = itsbinotstraight::biword();
    let replacement = itsbinotstraight::replacement(&biword, &mut rand::thread_rng());
    println!("{} → {}", biword, replacement);
}
