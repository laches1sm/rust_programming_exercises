use std::collections::HashMap;

fn main() {
    let mut quote_map = HashMap::new();
    quote_map.insert("Obi Wan Kenobi", "These aren't the droids you're looking for");
    quote_map.insert("Derek Zoolander", "What is this, a school for ants!?");
    quote_map.insert("Julius Caesar", "Et tu, Brutus?");
    quote_map.insert("Glitterhoof", "Neigh");

    for (&quote, &value) in quote_map.iter(){
        println!("{} said \"{}\"", quote, value)
    }

}


