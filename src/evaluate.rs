use instance::Instance;
use parser;
use std::collections::HashMap;
use std::f64;

pub struct Evaluator<'a> {
    instance: &'a Instance<'a>,
    weight: u32,
    profit: u32,
    time: f64,
    caught_items: u32,
    asked_items: u32,
    spw: f64,
    okay: bool,
}

#[derive(Debug)]
pub struct CalcResult {
    pub time: f64,
    pub weight: u32,
    pub profit: u32,
    pub okay: bool,
}

impl<'a> Evaluator<'a> {
    pub fn unvisit_city(&mut self, city: &u32, asked_items_hash: &HashMap<u32, bool>) {
        // add weight and profit and caught
        let (w, p, c) = self.instance.visit_city(*city, &asked_items_hash);
        self.weight -= w;
        self.profit -= p;
        self.caught_items -= c;
    }
    pub fn visit_city(&mut self, city: &u32, asked_items_hash: &HashMap<u32, bool>) {
        // add weight and profit and caught
        let (w, p, c) = self.instance.visit_city(*city, &asked_items_hash);
        self.weight += w;
        self.profit += p;
        self.caught_items += c;
    }

    pub fn set_asked_items(&mut self, asked: u32) {
        self.asked_items = asked;
    }

    pub fn check_okay_status(&mut self) -> bool {
        let okay = self.caught_items == self.asked_items
            && self.time <= self.instance.get_max_time().into()
            && self.weight <= self.instance.get_capacity_of_knapsack();

        let _debug = false;
        if okay == false && _debug {
            println!("Not okay!");
            println!("caught_items {}", self.caught_items);
            println!("asked_items {}", self.asked_items);
            println!("time {}", self.time);
            println!("max_time {}", self.instance.get_max_time());
            println!("weight {}", self.weight);
            println!(
                "get_capacity_of_knapsack {}",
                self.instance.get_capacity_of_knapsack()
            );
        }

        self.okay = okay;

        okay
    }

    pub fn walk_to_other_city(&mut self, last_city: &u32, next_city: &u32) {
        //walk

        // println!("walking... {}, {}, {}", self.time, last_city, next_city);

        let distance = self.instance.get_distance(last_city, next_city);
        // println!("distance {}", distance);
        let speed: f64 = self.instance.get_max_speed() - (self.weight as f64) * self.spw;
        // println!("speed {}", speed);

        if speed < 0.0 {
            self.time = f64::INFINITY;
            return;
        }

        self.time += (distance as f64) / speed;
        // println!("walked... {}", self.time);
    }

    pub fn calc(&mut self, solution: &parser::SolutionFile) -> CalcResult {
        let mut asked_items_hash: HashMap<u32, bool> = HashMap::new();
        for asked in solution.items.iter() {
            asked_items_hash.insert(*asked, true);
        }

        self._calc(&asked_items_hash, &solution.route)
    }

    pub fn _calc(&mut self, asked_items_hash: &HashMap<u32, bool>, route: &Vec<u32>) -> CalcResult {
        self.asked_items = asked_items_hash.len() as u32;

        let mut route_iterator = route.iter();
        let mut last_city = route_iterator.next();
        let mut next_city = route_iterator.next();
        while next_city.is_some() {
            // add weight and profit and caught
            self.visit_city(last_city.unwrap(), &asked_items_hash);

            self.walk_to_other_city(last_city.unwrap(), next_city.unwrap());

            // itera de novo
            last_city = next_city;
            next_city = route_iterator.next();
        }

        // add weight and profit and caught
        self.visit_city(last_city.unwrap(), &asked_items_hash);

        self.check_okay_status();

        CalcResult {
            time: self.time,
            weight: self.weight,
            profit: self.profit,
            okay: self.okay,
        }
    }

    pub fn _reset(&mut self) {
        self.weight = 0;
        self.profit = 0;
        self.time = 0.0;
        self.asked_items = 0;
        self.caught_items = 0;
        self.okay = true;
    }

    pub fn new(instance: &'a Instance) -> Evaluator<'a> {
        Evaluator {
            instance: instance,
            weight: 0,
            profit: 0,
            time: 0.0,
            asked_items: 0,
            caught_items: 0,
            okay: true,
            spw: instance.speed_descresc_per_weight(),
        }
    }
}

#[cfg(test)]
mod test_full {

    use std::fs::File;
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn integration() {
        let mut f = File::open("./input-a/instances/ex4-n5_1.thop").expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let instance = parser::instance::parse(&contents);
        let super_file = Instance::new(&instance);

        let mut ev = Evaluator::new(&super_file);

        let mut h1 = HashMap::new();
        h1.insert(1, true);
        h1.insert(2, true);

        ev.visit_city(&2, &h1);
        assert_eq!(ev.weight, 5);
        assert_eq!(ev.profit, 50);
        assert_eq!(ev.caught_items, 2);

        ev._reset();

        ev.walk_to_other_city(&1, &2);
        assert_eq!(ev.time, 5.0);

        ev._reset();

        let mut h2 = HashMap::new();
        h2.insert(1, true);

        ev.visit_city(&2, &h2);
        ev.walk_to_other_city(&1, &2);
        assert_eq!(ev.time, 12.5);
    }
}
