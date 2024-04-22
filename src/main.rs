use std::io::{self, Read};
mod graph_coloring;


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    match graph_coloring::reduce_to_casting_problem(&input) {
        Ok(casting_input) => println!("{}", casting_input),
        Err(e) => println!("Error parsing input: {}", e),
    }
}
