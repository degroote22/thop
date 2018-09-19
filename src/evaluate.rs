use parser;

struct Evaluator<'a> {
    instance: &'a parser::SuperFile<'a>,
    solution: &'a parser::SolutionFile,
}

#[derive(Debug)]
struct CalcResult {
    time: f64,
    weight: u32,
    profit: u32,
}

impl<'a> Evaluator<'a> {
    fn calc(&self) -> CalcResult {
        let mut route = self.solution.route.iter();
        let mut last_city = route.next();
        let mut next_city = route.next();
        let mut weight: u32 = 0;
        let mut profit: u32 = 0;
        let mut time: f64 = 0.0;
        while next_city.is_some() {
            println!("{:?}", next_city);
            println!("{:?}", last_city);
            let distance = self
                .instance
                .get_distance(last_city.unwrap(), next_city.unwrap());
            println!("distance {:?}", distance);

            let speed: f64 = self.instance.get_max_speed()
                - (weight as f64) * self.instance.speed_descresc_per_weight();

            time += (distance as f64) / speed;

            // add weight and profit

            let (w, p) = self
                .instance
                .visit_city(*next_city.unwrap(), &self.solution.items);
            weight += w;
            profit += p;

            last_city = next_city;
            next_city = route.next();
        }
        CalcResult {
            time,
            weight,
            profit,
        }
    }

    fn new(instance: &'a parser::SuperFile, solution: &'a parser::SolutionFile) -> Evaluator<'a> {
        Evaluator {
            instance: instance,
            solution,
        }
    }
}
// fn get_items_hash(items: &Vec<u32>) -> HashMap<u32, bool> {
//     let mut hash = HashMap::new();

//     for item in items {
//         hash.insert(*item, true);
//     }
//     hash
// }

pub fn evaluate(instance: parser::SuperFile, solution: parser::SolutionFile) {
    // println!("{:?}", instance.instance);
    println!("{:?}", solution);

    let ev = Evaluator::new(&instance, &solution);
    let c = ev.calc();
    println!("Time: {}", c.time);
    println!("Weight: {}", c.profit);
    println!("Profit: {}", c.weight);
}
