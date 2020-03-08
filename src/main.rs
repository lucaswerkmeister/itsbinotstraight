fn main() {
    let biword = itsbinotstraight::biword();
    let replacement = itsbinotstraight::replacement(&biword);
    println!("{} → {}", biword, replacement);
}
