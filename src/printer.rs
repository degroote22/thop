use evaluate;
use inputs;
use instance;
use parser;
use sa;
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use vns;

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
    route: Vec<u32>,
    asked_items: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultsPartOne {
    name: String,
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

fn chunk<T>(arr: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut i = 0;

    let mut v = Vec::new();

    for _ in 0..size {
        v.push(Vec::new());
    }

    for item in arr {
        v[i].push(item);
        i = (i + 1) % size;
    }

    v
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
        let mut _pr107: Vec<String> = contents
            .lines()
            .map(|line| pr107_base.to_owned() + line)
            .into_iter()
            .collect();

        let a280_base = "./input-b/instances/a280-thop/";
        let mut f = File::open("./src/inputs/p1/a280.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let mut _a280: Vec<String> = contents
            .lines()
            .map(|line| a280_base.to_owned() + line)
            .into_iter()
            .collect();

        let dsj1000_base = "./input-b/instances/dsj1000-thop/";
        let mut f = File::open("./src/inputs/p1/dsj1000.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let mut _dsj1000: Vec<String> = contents
            .lines()
            .map(|line| dsj1000_base.to_owned() + line)
            .into_iter()
            .collect();

        eli51.append(&mut _pr107);
        eli51.append(&mut _a280);
        eli51.append(&mut _dsj1000);
        eli51
    };
    let mut results: Vec<PartOneResult> = vec![];
    let length = itens.len();
    let chunks = chunk(itens, 4);
    let (tx, rx) = mpsc::channel();

    for chunk in chunks {
        let tx = tx.clone();
        thread::spawn(move || {
            for (index, i) in chunk.iter().enumerate() {
                let now = Instant::now();

                let mut f = File::open(i).expect("file not found");
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .expect("something went wrong reading the file");

                let instance_file = parser::instance::parse(&contents);
                let instance = instance::Instance::new(&instance_file);

                let ans = {
                    if false {
                        sa::sa(&instance, 25000, 250.0)
                    } else {
                        vns::vns(&instance)
                    }
                };

                let (route, hash) = ans;

                let result = evaluate::Evaluator::new(&instance)._calc(&hash, &route);

                if !result.okay {
                    panic!("Retornando rota inválida {}", i);
                }
                let new_now = Instant::now();

                let asked_items = {
                    let mut v = Vec::new();
                    for (key, _) in hash.iter() {
                        v.push(*key);
                    }
                    v
                };

                let r = PartOneResult {
                    name: i.to_string(),
                    okay: result.okay,
                    time: result.time,
                    weight: result.weight,
                    profit: result.profit,
                    index: index as u32,
                    execution_time_sub: new_now.duration_since(now).subsec_nanos(),
                    execution_time: new_now.duration_since(now).as_secs(),
                    route,
                    asked_items,
                };

                tx.send(r).unwrap();
            }
        });
    }

    for _ in 0..length {
        let received = rx.recv().unwrap();

        println!("received {}/{}", results.len(), length);

        results.push(received);
    }

    let mut file_to_write = File::create("vnsfbi.json").unwrap();

    let json = ResultsPartOne {
        name: "vnsfbi".to_string(),
        r: results,
    };
    println!("{}", json.json());

    file_to_write.write_all(json.json().as_bytes()).unwrap();
}
