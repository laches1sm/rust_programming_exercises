use rand::seq::SliceRandom;
use text_io::read;


fn main() {
    let mut flag: bool = true;
    let mut greet_vector: Vec<String> = Vec::new();
    greet_vector.push("Dia dhuit".to_string());
    greet_vector.push("Annyeonghaseo".to_string());
    greet_vector.push("Hello".to_string());
    greet_vector.push("Why hello there".to_string());


    while flag {
        println!("Hello, what is your name?");
        let name: String = read!();
        println!(
            "{:?}, {}! Nice to meet you",
            greet_vector.choose(&mut rand::thread_rng()),
            name
        );
        let c: String = read!();
        println!("Want another greeting?");
        if c == "yes".to_string() || c == "y".to_string() {
            continue;
        } else {
            flag = false;
        }
    }
}
