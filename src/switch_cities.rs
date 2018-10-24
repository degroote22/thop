use evaluate;
use instance;
use std::collections::HashMap;

fn make_pairs(route: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut v = Vec::new();

    let final_city = route[route.len() - 1];

    let mut last_city = route[1];
    for next_city in route.iter().skip(2) {
        if *next_city == final_city {
            continue;
        }

        v.push((last_city, *next_city));
        last_city = *next_city;
    }
    v
}

fn switch_in_vec(route: &Vec<u32>, switch: (u32, u32)) -> Vec<u32> {
    let mut new_route = route.clone();

    let mut first_position: Option<usize> = None;
    let mut last_position: Option<usize> = None;

    for (index, item) in new_route.iter().enumerate() {
        if *item == switch.0 {
            first_position = Some(index);
        }

        if *item == switch.1 {
            last_position = Some(index);
        }

        if last_position.is_some() && first_position.is_some() {
            break;
        }
    }

    new_route[last_position.unwrap()] = switch.0;
    new_route[first_position.unwrap()] = switch.1;

    new_route
}

#[cfg(test)]
mod switch_cities_test {

    use super::*;

    #[test]
    fn test_make_pairs() {
        let route = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(make_pairs(&route), vec![(2, 3), (3, 4), (4, 5)]);
    }

    #[test]
    fn test_switch_in_vec() {
        let route = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(switch_in_vec(&route, (2, 3)), vec![1, 3, 2, 4, 5, 6]);
        assert_eq!(switch_in_vec(&route, (3, 2)), vec![1, 3, 2, 4, 5, 6]);
        assert_eq!(switch_in_vec(&route, (2, 5)), vec![1, 5, 3, 4, 2, 6]);
        assert_eq!(switch_in_vec(&route, (5, 2)), vec![1, 5, 3, 4, 2, 6]);
    }
}

// struct SwitchCitiesPartial {
//     switch: (u32, u32),
//     profit: u32,
// }

pub fn switch_cities(
    instance: &instance::Instance,
    mut route: Vec<u32>,
    hash: &HashMap<u32, bool>,
    first_improvement: bool,
) -> Vec<u32> {
    // pra cada par possível de trocar, calcula-se o valor trocado
    let mut ev = evaluate::Evaluator::new(&instance);
    let mut old_time = {
        ev._reset();
        let result = ev._calc(hash, &route);
        result.time
    };

    let mut switch_final: Option<(u32, u32)> = None;

    let switches = make_pairs(&route);

    for switch in switches {
        let new_route = switch_in_vec(&route, switch);
        ev._reset();
        let new_result = ev._calc(hash, &new_route);
        let new_time = new_result.time;
        // println!(
        //     "switch {:?} oldp {} newp {} okay {}",
        //     switch, old_time, new_time, new_result.okay
        // );
        if new_result.okay && new_time < old_time {
            // println!("found new time {} old time {}", new_time, old_time);
            if first_improvement {
                return new_route;
            }

            old_time = new_time;
            switch_final = Some(switch);
        }
    }

    if switch_final.is_some() {
        route = switch_in_vec(&route, switch_final.unwrap());
    }

    route
}
