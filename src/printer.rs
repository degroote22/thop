use evaluate;
use greedy;
use inputs;
use instance;
use parser;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct PartOneResult {
    time: f64,
    weight: u32,
    profit: u32,
    okay: bool,
    name: String,
    index: u32,
    execution_time_sub: u32,
    execution_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultsPartOne {
    r: Vec<PartOneResult>,
}

impl ResultsPartOne {
    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
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
    let itens: Vec<String> = {
        let eli51_base = "./input-b/instances/eil51-thop/";
        let mut f = File::open("./src/inputs/p1/eli51.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let mut eli51: Vec<String> = contents
            .lines()
            .map(|line| eli51_base.to_owned() + line)
            .into_iter()
            .collect();

        let pr107_base = "./input-b/instances/pr107-thop/";
        let mut f = File::open("./src/inputs/p1/pr107.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let mut pr107: Vec<String> = contents
            .lines()
            .map(|line| pr107_base.to_owned() + line)
            .into_iter()
            .collect();

        let a280_base = "./input-b/instances/a280-thop/";
        let mut f = File::open("./src/inputs/p1/a280.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let mut a280: Vec<String> = contents
            .lines()
            .map(|line| a280_base.to_owned() + line)
            .into_iter()
            .collect();

        let dsj1000_base = "./input-b/instances/dsj1000-thop/";
        let mut f = File::open("./src/inputs/p1/dsj1000.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let mut dsj1000: Vec<String> = contents
            .lines()
            .map(|line| dsj1000_base.to_owned() + line)
            .into_iter()
            .collect();

        eli51.append(&mut pr107);
        eli51.append(&mut a280);
        eli51.append(&mut dsj1000);
        eli51
    };
    let mut results: Vec<PartOneResult> = vec![];

    for (index, i) in itens.iter().enumerate() {
        let now = Instant::now();

        let mut f = File::open(i).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let instance_file = parser::instance::parse(&contents);
        let instance = instance::Instance::new(&instance_file);

        let (route, hash) = greedy::greedy(&instance);

        let result = evaluate::Evaluator::new(&instance)._calc(&hash, &route);

        // println!("Evaluating {}", i);
        // println!("Profit: {}", result.profit);
        // println!("Time: {}", result.time);
        // println!("Okay: {}", result.okay);
        // println!("");
        if !result.okay {
            panic!("Retornando rota inválida");
        }
        let new_now = Instant::now();

        let r = PartOneResult {
            name: i.to_string(),
            okay: result.okay,
            time: result.time,
            weight: result.weight,
            profit: result.profit,
            index: index as u32,
            execution_time_sub: new_now.duration_since(now).subsec_nanos(),
            execution_time: new_now.duration_since(now).as_secs(),
        };

        results.push(r);
    }

    // println!("printing profit");
    // for result in results.iter() {
    //     println!("{}", result.profit)
    // }
    // println!("");
    // println!("printing time");
    // for result in results.iter() {
    //     println!("{}", result.time)
    // }

    let json = ResultsPartOne { r: results };

    println!("{}", json.json());
}
