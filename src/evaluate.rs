use parser;
use std::collections::HashMap;

struct Evaluator<'a> {
    instance: &'a parser::THOPFile,
    solution: &'a parser::SolutionFile,
}

impl<'a> Evaluator<'a> {
    fn calc(&self) {
        let mut route = self.solution.route.iter();
        let mut last_city = route.next();
        let mut city = route.next();
        let mut cost = 0.0;
        while city.is_some() {
            let new_cost = 0.0;

            last_city = city;
            city = Some(&0);

            cost += new_cost;
        }
    }

    fn get_distance(&self, from: u32, to: u32) {}
}

pub fn evaluate(instance: parser::THOPFile, solution: parser::SolutionFile) {
    println!("{:?}", instance);
    println!("{:?}", solution);

    let first_city = solution.route.get(0).unwrap();
    // let cost = _eval_route_item(instance, solution, *first_city);
    println!("Cost: {}", first_city);

    let mut ev = Evaluator {
        instance: &instance,
        solution: &solution,
    };
    ev.calc();
}
