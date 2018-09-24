use evaluate;
use greedy;
use inputs;
use instance;
use parser;
use std::fs::File;
use std::io::prelude::*;

pub fn print_results_part_0() {
    for (i, s) in inputs::INSTANCES_P0.iter() {
        let mut f = File::open(i).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let mut f2 = File::open(s).expect("file not found");
        let mut contents2 = String::new();
        f2.read_to_string(&mut contents2)
            .expect("something went wrong reading the file");

        let solution = parser::solution::parse(&contents2);
        let instance_file = parser::instance::parse(&contents);
        let instance = instance::Instance::new(&instance_file);

        let result = evaluate::Evaluator::new(&instance).calc(&solution);

        println!("Evaluating {} {}", i, s);
        println!("Profit: {}", result.profit);
        println!("Time: {}", result.time);
        println!("Okay: {}", result.okay);
        println!("");
    }
}

pub fn print_results_part_1() {
    for (i, _s) in inputs::INSTANCES_P0.iter() {
        let mut f = File::open(i).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let instance_file = parser::instance::parse(&contents);
        let instance = instance::Instance::new(&instance_file);

        let (route, hash) = greedy::greedy(&instance);

        let result = evaluate::Evaluator::new(&instance)._calc(&hash, &route);

        println!("Evaluating {}", i);
        println!("Profit: {}", result.profit);
        println!("Time: {}", result.time);
        println!("Okay: {}", result.okay);
        println!("");
    }
}
