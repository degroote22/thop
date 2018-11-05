use change_item;
use evaluate;
use greedy;
use insert_city;
use instance;
use local_search;
use std::collections::HashMap;
use std::time::Instant;
use switch_cities;

#[derive(Debug)]
enum Neighboorhood {
    TwoOpt,
    ChangeItem,
    SwitchCities,
    InsertCity,
}

impl Neighboorhood {
    pub fn next(&self) -> Option<Neighboorhood> {
        match self {
            Neighboorhood::TwoOpt => Some(Neighboorhood::ChangeItem),
            Neighboorhood::ChangeItem => Some(Neighboorhood::SwitchCities),
            Neighboorhood::SwitchCities => Some(Neighboorhood::InsertCity),
            Neighboorhood::InsertCity => None,
        }
    }

    pub fn first() -> Neighboorhood {
        Neighboorhood::TwoOpt
    }

    pub fn search(
        &self,
        instance: &instance::Instance,
        route: &Vec<u32>,
        hash: &HashMap<u32, bool>,
    ) -> (Vec<u32>, HashMap<u32, bool>) {
        let first_improvement = false;
        // println!("search {:?}", self);
        match self {
            Neighboorhood::TwoOpt => {
                local_search::two_opt(&instance, route.clone(), hash.clone(), first_improvement)
            }
            Neighboorhood::ChangeItem => (
                route.clone(),
                change_item::change_item(&instance, &route, hash.clone(), first_improvement),
            ),
            Neighboorhood::SwitchCities => (
                switch_cities::switch_cities(&instance, route.clone(), &hash, first_improvement),
                hash.clone(),
            ),
            Neighboorhood::InsertCity => (
                insert_city::insert_city(&instance, route.clone(), &hash, first_improvement),
                hash.clone(),
            ),
        }
    }
}

pub fn vns(instance: &instance::Instance) -> (Vec<u32>, HashMap<u32, bool>) {
    let (route, hash) = greedy::greedy(&instance);
    let mut old_route = route.clone();
    let mut old_hash = hash.clone();
    let mut neighboorhood = Neighboorhood::first();
    let mut ev = evaluate::Evaluator::new(&instance);
    let now = Instant::now();

    loop {
        let new_now = Instant::now();
        let execution_time = new_now.duration_since(now).as_secs();

        if execution_time > 60 {
            return (old_route, old_hash);
        }

        let (new_route, new_hash) = neighboorhood.search(&instance, &old_route, &old_hash);

        ev._reset();
        let old_result = ev._calc(&old_hash, &old_route);

        ev._reset();
        let new_result = ev._calc(&new_hash, &new_route);

        if new_result.profit > old_result.profit || (new_result.time - old_result.time).abs() > 0.01
        {
            // println!("houve melhoria");
            // diferente, houve melhoria
            old_route = new_route;
            old_hash = new_hash;
            neighboorhood = Neighboorhood::first();
        } else {
            // igual, troca a vizinhanca

            let next = neighboorhood.next();
            match next {
                Some(n) => neighboorhood = n,
                None => {
                    // let mut ev = evaluate::Evaluator::new(&instance);
                    // let result = ev._calc(&old_hash, &old_route);
                    // assert!(result.okay);
                    return (old_route, old_hash);
                }
            }
        }
    }
}
