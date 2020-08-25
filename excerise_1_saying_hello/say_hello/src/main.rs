use rand::seq::SliceRandom;
use text_io::read;


fn main() {
    let mut flag: bool = true;
    let mut greet_vector = vec!["Dia dhuit", "Annyeonghaseo", "Hello", "Why hello there"];
    let mut rng = rand::thread_rng();

    while flag {
        println!("Hello, what is your name?");
        let name: String = read!();
        println!(
            "{}, {}! Nice to meet you",
            greet_vector.choose(&mut rng).unwrap(),
            name
        );
        let c: String = read!();
        let c = c.trim();
        println!("Want another greeting?");
        if c == "yes" || c == "y" {
            continue;
        } else {
            flag = false;
        }
    }
}
