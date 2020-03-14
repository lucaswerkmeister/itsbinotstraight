fn main() {
    let mut rng = rand::thread_rng();
    let biword = itsbinotstraight::biword(&mut rng);
    let phrase = itsbinotstraight::phrase(&biword, &mut rng);
    println!("{}", phrase);
}
