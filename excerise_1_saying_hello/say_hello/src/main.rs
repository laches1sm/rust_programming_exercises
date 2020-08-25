use rand::seq::SliceRandom;
use text_io::read;


fn main() {
    let mut flag: bool = true;
    let mut greet_vector: Vec<String> = Vec::new();
    greet_vector.push("Dia dhuit".parse().unwrap());
    greet_vector.push("Annyeonghaseo".parse().unwrap());
    greet_vector.push("Hello".parse().unwrap());
    greet_vector.push("Why hello there".parse().unwrap());

    println!("Hello, what is your name?");

    while flag {
        let name: String = read!();
        println!(
            "{:?}, {}! Nice to meet you",
            greet_vector.choose(&mut rand::thread_rng()),
            name
        );
        let c: String = read!();
        println!("Want another greeting?");
        if c == "yes".parse().unwrap() || c == "y".parse().unwrap() {
            continue;
        } else {
            flag = false;
        }
    }
}
