use text_io::read;

fn main() {
   println!("What is the input string?");
   let input_str: String = read!();
   let input_str = input_str.trim();
   if input_str.len() < 1 || input_str == "" {
      println!("Input should not be empty.")
   }
   println!("{} has {} characters.", input_str, input_str.len());


}
