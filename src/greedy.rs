use evaluate;
use instance;
use std::collections::HashMap;

pub fn greedy(instance: &instance::Instance) -> (Vec<u32>, HashMap<u32, bool>) {
    // iniciar na cidade 1
    let mut city: u32 = 1;
    let mut asked_items_hash: HashMap<u32, bool> = HashMap::new();
    let mut ev = evaluate::Evaluator::new(&instance);
    let mut route: Vec<u32> = vec![];
    let mut asked_items_number: u32 = 0;
    let mut black_list: HashMap<u32, bool> = HashMap::new();

    loop {
        route.push(city);
        black_list.insert(city, true);
        // escolher um item com o menor peso se tiver
        let items_in_city = instance.get_items_in_city(&city);
        let item = match items_in_city {
            Some(items) => Some(items.iter().min_by_key(|x| x.profit / x.weight).unwrap()),
            None => None,
        };

        if item.is_some() {
            asked_items_hash.insert(item.unwrap().index, true);
            asked_items_number += 1;
        }
        ev.visit_city(&city, &asked_items_hash);
        ev.set_asked_items(asked_items_number);

        // se o item excedeu o peso, tira ele
        let okay = ev.check_okay_status();
        if !okay {
            ev.unvisit_city(&city, &asked_items_hash);
            asked_items_hash.remove_entry(&item.unwrap().index);
            asked_items_number = asked_items_hash.len() as u32;
            ev.set_asked_items(asked_items_number);
        }

        // ir para a cidade mais próxima
        let closest = instance.closest_city(&city, &black_list);

        if closest.is_none() {
            // println!("Nao há cidade para ir");
            break;
        } else {
            let (city_index, _distance) = closest.unwrap();

            ev.walk_to_other_city(&city, &city_index);
            let okay = ev.check_okay_status();
            if !okay {
                // println!("Nao há como ir para a próxima cidade, tempo excedido");

                break;
            }
            city = city_index;
        }
    }

    // remove cidades desnecessarias no fim da rota
    // println!("Antes de limpar haviam {} cidades na rota", route.len());
    loop {
        let last_city = *route.get(route.len() - 1).unwrap();

        let items_in_city = instance.get_items_in_city(&last_city);
        match items_in_city {
            Some(items) => {
                // tem item na cidade, vamo ver se pegou
                let has_taken_some = {
                    let mut has = false;
                    for item in items {
                        if asked_items_hash.contains_key(&item.index) {
                            has = true;
                        }
                    }
                    has
                };
                if has_taken_some {
                    // pegou um item, entao essa parte da rota é útil
                    break;
                } else {
                    // nao pegou item, parte inutil no fim da rota
                    route.pop();
                }
            }
            None => {
                // nem tinha item na cidade, entao nao pegou nenhum item
                route.pop();
            }
        };
    }

    (route, asked_items_hash)
}
