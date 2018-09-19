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
    okay: bool,
}

impl<'a> Evaluator<'a> {
    fn calc(&self) -> CalcResult {
        let mut weight: u32 = 0;
        let mut profit: u32 = 0;
        let mut time: f64 = 0.0;
        let mut caught_items = 0;

        let spw = self.instance.speed_descresc_per_weight();

        let mut route = self.solution.route.iter();
        let mut last_city = route.next();
        let mut next_city = route.next();

        while next_city.is_some() {
            let distance = self
                .instance
                .get_distance(last_city.unwrap(), next_city.unwrap());

            let speed: f64 = self.instance.get_max_speed() - (weight as f64) * spw;

            time += (distance as f64) / speed;

            // add weight and profit and caught
            let (w, p, c) = self
                .instance
                .visit_city(*last_city.unwrap(), &self.solution.items);
            weight += w;
            profit += p;
            caught_items += c;
            // itera de novo
            last_city = next_city;
            next_city = route.next();
        }

        // add weight and profit and caught
        let (w, p, c) = self
            .instance
            .visit_city(*last_city.unwrap(), &self.solution.items);
        weight += w;
        profit += p;
        caught_items += c;

        let okay = (caught_items as usize) == self.solution.items.len()
            && time < self.instance.get_max_time().into()
            && weight < self.instance.get_capacity_of_knapsack();

        if okay == false {
            println!("Not okay!");
            println!("caught_items {}", caught_items);
            println!("asked_items {}", self.solution.items.len());
            println!("time {}", time);
            println!("max_time {}", self.instance.get_max_time());
            println!("weight {}", weight);
            println!(
                "get_capacity_of_knapsack {}",
                self.instance.get_capacity_of_knapsack()
            );
            println!("");
        }

        CalcResult {
            time,
            weight,
            profit,
            okay,
        }
    }

    fn new(instance: &'a parser::SuperFile, solution: &'a parser::SolutionFile) -> Evaluator<'a> {
        Evaluator {
            instance: instance,
            solution,
        }
    }
}

pub fn evaluate(instance: parser::SuperFile, solution: parser::SolutionFile) {
    let ev = Evaluator::new(&instance, &solution);
    let c = ev.calc();
    println!("Time: {}", c.time);
    println!("Weight: {}", c.weight);
    println!("Profit: {}", c.profit);
    println!("Okay: {}", c.okay);
}
