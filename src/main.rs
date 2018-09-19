#[macro_use]
extern crate lazy_static;
extern crate regex;
mod parser;
use parser::parse_instance;
use parser::parse_solution;
mod evaluate;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // "./input/instances/ex4-n5_1.thop"
    let mut f = File::open(args.get(1).unwrap()).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // "./input/solutions/ex4-n5_a.thop.sol"
    let mut f2 = File::open(args.get(2).unwrap()).expect("file not found");

    let mut contents2 = String::new();
    f2.read_to_string(&mut contents2)
        .expect("something went wrong reading the file");

    let instance = parse_instance(&contents);
    let solution = parse_solution(&contents2);

    evaluate::evaluate(instance, solution);
}
