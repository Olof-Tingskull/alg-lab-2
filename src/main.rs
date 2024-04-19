use std::io::{self, Read};
mod casting_problem;
use std::env;

fn no_instance() {
    casting_problem::run("
        5 
        5
        3

        3 - 1 2 3 
        2 - 2 3 
        2 - 1 3 
        1 - 2 
        3 - 1 2 3 

        2 - 1 2
        2 - 1 2
        3 - 1 3 4
        2 - 3 5
        3 - 2 3 5
    ",)
}

fn yes_instance() {
    casting_problem::run("
        6 
        5 
        4

        3 - 1 3 4
        2 - 2 3
        2 - 1 3
        1 - 2
        4 - 1 2 3 4
        2 - 1 4

        3 - 1 2 6
        3 - 2 3 5
        3 - 2 4 6
        3 - 2 3 6
        2 - 1 6 
    ")
}

fn smallest_problem() {
    casting_problem::run("
        2
        1
        2

        1 - 1
        1 - 2

        2 - 1 2
    ");
}

fn reduce() {
    casting_problem::reduce_to_graph_coloring("
        5 
        5
        3

        3 - 1 2 3 
        2 - 2 3 
        2 - 1 3 
        1 - 2 
        3 - 1 2 3 

        2 - 1 2
        2 - 1 2
        3 - 1 3 4
        2 - 3 5
        3 - 2 3 5
    ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "no" => no_instance(),
            "yes" => yes_instance(),
            "smallest" => smallest_problem(),
            "reduce" => reduce(),
            _ => println!("Invalid argument. Please use 'no', 'yes', 'smallest', or 'reduce'."),
        }
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        casting_problem::reduce_to_graph_coloring(&input);
    }
}