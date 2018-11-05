use greedy;
// use insert_city;
use instance;
// use local_search;
use evaluate;
use rand::prelude::*;
use std::collections::HashMap;

fn temperature(x: f64, initial_temperature: f64) -> f64 {
    initial_temperature / (x + 1.0)
}

fn get_random_insertable_city(instance: &instance::Instance, route: &Vec<u32>) -> Option<u32> {
    let mut rng = thread_rng();
    if route.len() as u32 >= instance.get_dimension() - 1 {
        return None;
    }
    'outer: loop {
        let random_city = rng.gen_range(2, instance.get_dimension());
        for city in route {
            if *city == random_city {
                continue 'outer;
            }
        }
        return Some(random_city);
    }
}

fn random_neighboor(
    instance: &instance::Instance,
    route: &Vec<u32>,
    hash: &HashMap<u32, bool>,
) -> Option<(Vec<u32>, HashMap<u32, bool>)> {
    let mut rng = thread_rng();

    let random_insertable_city = get_random_insertable_city(&instance, &route);

    if let None = random_insertable_city {
        // nao há como inserir nada, retorna
        return None;
    }
    let mut new_route = route.clone();
    // let random_city = cities.get(x as usize);

    let random_city_from_route = rng.gen_range(2, route.len() - 1);

    let old_city = new_route[random_city_from_route];

    new_route[random_city_from_route] = random_insertable_city.unwrap();

    let mut new_hash = hash.clone();

    // remove os itens antigos
    let items_in_old_city = instance.get_items_in_city(&old_city);
    if let Some(items) = items_in_old_city {
        for i in items {
            if hash.contains_key(&i.index) {
                new_hash.remove(&i.index);
            }
        }
    }

    let items_in_new_city = instance.get_items_in_city(&random_insertable_city.unwrap());

    if let Some(items) = items_in_new_city {
        let mut ev = evaluate::Evaluator::new(&instance);
        for i in items {
            ev._reset();
            // tenta inserir
            new_hash.insert(i.index, true);
            // testa se tá ok
            let result = ev._calc(&new_hash, &new_route);

            // se não tá, remove e sai do loop
            if !result.okay {
                new_hash.remove(&i.index);
                break;
            }
        }
    }

    Some((new_route, new_hash))
}

fn probability_function(diff: i32, temp: f64) -> bool {
    let mut rng = thread_rng();
    let random_number: f64 = rng.gen(); // random number in range [0, 1)
    let div = (diff as f64) / temp;
    let result = random_number >= div;
    // println!(
    //     "prob func x: {}, diff: {}, temp: {}, div: {}, result: {}",
    //     random_number, diff, temp, div, result
    // );
    result
}

pub fn sa(
    instance: &instance::Instance,
    kmax: u32,
    initial_temperature: f64,
) -> (Vec<u32>, HashMap<u32, bool>) {
    // println!("starting sa");
    // let s = s0
    let (mut route, mut hash) = greedy::greedy(&instance);
    let mut ev = evaluate::Evaluator::new(&instance);

    let mut result = ev._calc(&hash, &route);
    //for k = 0 to kmax
    for k in 0..kmax {
        // t = temperature(k/kmax)
        let t = temperature(k as f64 / kmax as f64, initial_temperature);

        // pick a random neighboor

        let _random_neighboor = random_neighboor(&instance, &route, &hash);
        if let None = _random_neighboor {
            // println!(
            //     "número máximo de cidades na rota, não há o que fazer {}",
            //     route.len()
            // );
            // println!("{:?}", route);
            return (route, hash);
        }
        let (new_route, new_hash) = _random_neighboor.unwrap();
        ev._reset();
        let new_result = ev._calc(&new_hash, &new_route);
        let diff: i32 = (result.profit as i32) - (new_result.profit as i32);
        if diff >= 0 && new_result.profit > result.profit {
            panic!("bug");
        }
        // se eh melhor ou se é aceito pela temperatura
        if new_result.okay && (diff < 0 || probability_function(diff, t)) {
            //atualiza
            result = new_result;
            route = new_route;
            hash = new_hash;
        }
    }

    (route, hash)
}
