fn main() {
    let biword = itsbinotstraight::biword();
    let phrase = itsbinotstraight::phrase(&biword, &mut rand::thread_rng());
    println!("{}", phrase);
}
