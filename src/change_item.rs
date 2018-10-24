use evaluate;
use instance;
use std::collections::HashMap;

struct GetBestItemsPartial {
    profit: u32,
    items: Vec<u32>,
}

fn get_current_addition(
    instance: &instance::Instance,
    route: &Vec<u32>,
    hash: &HashMap<u32, bool>,
    c: &u32,
) -> Option<GetBestItemsPartial> {
    let items = instance.get_items_in_city(c);

    if let Some(it) = items {
        let mut v = Vec::new();
        let mut new_hash = hash.clone();
        let mut ev = evaluate::Evaluator::new(&instance);
        let mut profit: u32 = 0;

        for item in it {
            if hash.contains_key(&item.index) {
                // se o item já tá no hash ele nao é retornado
                continue;
            }
            // tenta adicionar o item
            new_hash.insert(item.index, true);
            ev._reset();
            // testa se tá ok
            let result = ev._calc(&new_hash, &route);
            if result.okay {
                v.push(item.index);
                profit = result.profit;
            } else {
                new_hash.remove(&item.index);
            }
        }

        // retorna o vetor se tiver adicionado algum item
        if v.len() == 0 {
            return None;
        } else {
            return Some(GetBestItemsPartial { profit, items: v });
        }
    } else {
        return None;
    }
}

fn get_best_items(
    instance: &instance::Instance,
    route: &Vec<u32>,
    hash: &HashMap<u32, bool>,
) -> Option<Vec<u32>> {
    let partial: Option<GetBestItemsPartial> = route.iter().fold(None, |p, c| {
        let current_addition = get_current_addition(&instance, &route, &hash, c);
        match p {
            Some(old) => match current_addition {
                Some(curr) => {
                    if curr.profit > old.profit {
                        return Some(curr);
                    } else {
                        return Some(old);
                    }
                }
                None => {
                    return Some(old);
                }
            },
            None => current_addition,
        }
    });

    partial.map(|i| i.items)
}

pub fn change_item(
    instance: &instance::Instance,
    route: &Vec<u32>,
    mut hash: HashMap<u32, bool>,
    first_improvement: bool,
) -> HashMap<u32, bool> {
    // 'outer: loop {
    let old_profit = {
        let mut ev = evaluate::Evaluator::new(&instance);
        let result = ev._calc(&hash, &route);
        result.profit
    };

    // pra cada item que tá sendo pedido
    'inner: for (item_index, _) in hash.clone().iter() {
        // tiro esse item da rota
        hash.remove(item_index);

        // escolho os melhores outros itens pra colocar em alguma cidade
        let best_items = get_best_items(instance, &route, &hash);

        if let Some(to_add_items) = best_items {
            for item in to_add_items.iter() {
                hash.insert(*item, true);
            }
            let new_profit = {
                let mut ev = evaluate::Evaluator::new(&instance);
                let result = ev._calc(&hash, &route);
                result.profit
            };
            if new_profit > old_profit {
                if first_improvement {
                    return hash;
                }
            // se houver tal itens, volta pro começo e itera pra cada item desde o início
            // continue 'outer;
            } else {
                for item in to_add_items {
                    hash.remove(&item);
                }
                // nao apaga esse item e continua o loop interior
                hash.insert(*item_index, true);
            }
        } else {
            // nao apaga esse item e continua o loop interior
            hash.insert(*item_index, true);
        }
    }

    // retorna a rota e os itens se executou o loop interior até o final
    return hash;
    // }
}
