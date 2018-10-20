use evaluate;
use instance;
use std::collections::HashMap;

#[derive(Debug)]
struct BestImprov {
    pub profit: u32,
    pub city: u32,
    pub items: Vec<u32>,
}

pub fn two_opt(
    instance: &instance::Instance,
    mut route: Vec<u32>,
    mut hash: HashMap<u32, bool>,
) -> (Vec<u32>, HashMap<u32, bool>) {
    let mut ev = evaluate::Evaluator::new(&instance);

    // pra cada cidade que está na rota
    for (index, old_city) in route.clone().iter().enumerate() {
        //nao pode trocar a primeira nem a última cidade da rota
        if *old_city == 1 || *old_city == instance.get_dimension() {
            continue;
        }

        let mut route_hash = HashMap::new();
        for city in route.iter() {
            route_hash.insert(city.clone(), true);
        }
        let switchable_cities: Vec<u32> = instance
            .get_cities()
            .iter()
            .filter(|&city| !route_hash.contains_key(&city.index))
            .map(|city| city.index)
            .collect();

        let old_profit = {
            ev._reset();
            let result = ev._calc(&hash, &route);
            result.profit
        };
        let old_items = {
            let mut v = Vec::new();
            let all_items = instance.get_items_in_city(old_city);
            if let Some(items) = all_items {
                for item in items {
                    if hash.contains_key(&item.index) {
                        v.push(item.index);
                    }
                }
            }
            v
        };
        let mut best_improv = BestImprov {
            city: *old_city,
            profit: old_profit,
            items: old_items,
        };

        // nao tentar trocar por uma cidade que já está na rota
        // println!("hash antes {:?}", hash);
        for city in switchable_cities.iter() {
            // remover os itens da cidade que tava

            for item in best_improv.items.iter() {
                // println!("removendo item {:?} da cidade {:?}", item, best_improv.city);
                hash.remove(item);
            }
            // println!("hash depois {:?}", hash);

            // trocar a cidade
            route[index] = *city;

            // testa se tá válido, se tiver tenta adicionar essa parte na rota
            // se tiver melhor o lucro
            ev._reset();
            let result = ev._calc(&hash, &route);

            if result.okay {
                // println!("trocou {} por {}", old_city, city);
                // se ainda tá válido, pega o item que tem o melhor valor por peso
                // TODO: ordernar por esse fator
                let mut caught_items = Vec::new();
                let maybe_items = instance.get_items_in_city(city);

                let mut current_profit = result.profit;
                if let Some(items) = maybe_items {
                    for item_to_catch in items {
                        // println!("testando se o item {:?} cabe", item_to_catch);
                        // se o item cabe
                        if ev.can_get_item(city, &item_to_catch.index) {
                            // println!("item {:?} cabe ", item_to_catch);
                            // println!("inserindo item {:?} da cidade {:?}", item_to_catch, city);
                            hash.insert(item_to_catch.index, true);
                            // ve se ele nao aumentou muito o peso e acabou fodendo o role
                            // atualiza o evaluator pra ele saber que tá pegando esse item (aumenta o peso e lucro)
                            ev._reset();
                            let item_insertion_result = ev._calc(&hash, &route);
                            if item_insertion_result.okay {
                                current_profit = item_insertion_result.profit;
                                // coloca isso pra passar pro bestimprov
                                caught_items.push(item_to_catch.index);
                            } else {
                                hash.remove(&item_to_catch.index);
                            }
                        }
                    }
                }

                // se o lucro com essas alterações é melhor do que antes, altera o best_improv
                if current_profit > best_improv.profit {
                    // println!("autalizando bi {:?}", best_improv);
                    best_improv.city = *city;
                    best_improv.profit = current_profit;
                    best_improv.items = caught_items;
                // println!("autalizou bi {:?}", best_improv);
                } else {
                    for item in caught_items {
                        hash.remove(&item);
                    }
                }
            }

            // adiciona os itens
            for item in best_improv.items.iter() {
                // println!(
                //     "FINALLLL inserindo item {:?} da cidade {:?}",
                //     item, best_improv.city
                // );

                hash.insert(*item, true);
            }
            // vai pra cidade do best improv
            route[index] = best_improv.city;

            // ev._reset();
            // let r = ev._calc(&hash, &route);
            // if !r.okay {
            //     panic!(
            //         "erro na hora de finalizar o two-opt com a cidade {} ia trocar pra {} hash {:?} old {:?} new {:?} ",
            //         old_city, city, hash, instance.get_items_in_city(old_city), instance.get_items_in_city(city)
            //     )
            // }
        }
    }

    (route.clone(), hash)
}

#[cfg(test)]
mod test_two_opt {
    use super::*;
    use greedy;
    use parser;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn integration() {
        let file_name =
            "./input-b/instances/eil51-thop/eil51_n147_bounded-strongly-corr_01_01.thop";

        let mut f = File::open(file_name).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let instance_file = parser::instance::parse(&contents);
        let instance = instance::Instance::new(&instance_file);

        let (route, hash) = greedy::greedy(&instance);
        let pre_result = evaluate::Evaluator::new(&instance)._calc(&hash, &route);

        println!("old route {:?} hash {:?}", route, hash);
        let (final_route, asked_items_hash) = two_opt(&instance, route, hash);
        println!("old route {:?} hash {:?}", final_route, asked_items_hash);

        let result = evaluate::Evaluator::new(&instance)._calc(&asked_items_hash, &final_route);
        assert!(pre_result.profit <= result.profit);
        assert!(result.okay);
    }

}
