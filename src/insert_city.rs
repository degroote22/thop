use evaluate;
use instance;
use std::collections::HashMap;
#[derive(Copy, Clone)]
struct BestInsertion {
    index: usize,
    city: u32,
    time: f64,
}

pub fn insert_city(
    instance: &instance::Instance,
    mut route: Vec<u32>,
    hash: &HashMap<u32, bool>,
    first_improvement: bool,
) -> Vec<u32> {
    let mut ev = evaluate::Evaluator::new(&instance);
    let mut route_hash = HashMap::new();
    for city in route.iter() {
        route_hash.insert(city.clone(), true);
    }
    let insertable_cities: Vec<u32> = instance
        .get_cities()
        .iter()
        .filter(|&city| !route_hash.contains_key(&city.index))
        .map(|city| city.index)
        .collect();

    let mut best_insertion: Option<BestInsertion> = None;

    for city in insertable_cities {
        // pra cada cidade possível de inserir

        // pra cada posicao onde se pode inserir
        for index in 1..route.len() {
            let mut new_route = route.clone();
            new_route.insert(index, city);
            ev._reset();
            let new_result = ev._calc(hash, &new_route);

            if new_result.okay {
                // println!("insert city okay");
                if first_improvement {
                    return new_route;
                }
                match best_insertion {
                    Some(bi) => {
                        if new_result.time < bi.time {
                            best_insertion = Some(BestInsertion {
                                time: new_result.time,
                                index,
                                city,
                            });
                        }
                    }
                    None => {
                        best_insertion = Some(BestInsertion {
                            time: new_result.time,
                            index,
                            city,
                        });
                    }
                }
            }
        }
    }

    if best_insertion.is_some() {
        // se há uma melhor insercao, insere na rota
        let bi = best_insertion.unwrap();
        route.insert(bi.index, bi.city);
    }

    route
}
