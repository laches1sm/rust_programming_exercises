use text_io::read;


fn main() {
    println!("Enter a noun");
    let noun: String = read!();
    let noun = noun.trim();
    println!("Enter a verb");
    let verb: String = read!();
    let verb = verb.trim();
    println!("Enter an adjective");
    let adjective: String = read!();
    let adjective = adjective.trim();
    println!("Enter an adverb");
    let adverb: String = read!();
    let adverb = adverb.trim();
    println!("I heard you like to {} your {} {} {} in the morning? That's amazing!", verb, adjective, noun, adverb);


    }



