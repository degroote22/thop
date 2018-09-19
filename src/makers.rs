use parser;
use std::collections::HashMap;

pub fn make_distance_matrix(coords: &Vec<parser::NodeCoordSection>) -> Vec<Vec<u32>> {
    let mut h = HashMap::new();
    for coord in coords.iter() {
        h.insert(coord.index, coord);
    }

    let len = coords.len(); //se aqui eh 4

    let mut accumulator = 2;
    let mut v = Vec::default();
    // de 1 a 3
    for a in 1..len {
        let mut line = Vec::default();
        // de 2 a 4
        for b in accumulator..(len + 1) {
            let xa = h.get(&(a as u32)).unwrap().x;
            let xb = h.get(&(b as u32)).unwrap().x;
            let ya = h.get(&(a as u32)).unwrap().y;
            let yb = h.get(&(b as u32)).unwrap().y;
            let sum = (xa - xb).powf(2.0) + (ya - yb).powf(2.0);
            let sqrt = sum.sqrt();
            let ceiled = sqrt.ceil();
            line.push(ceiled as u32);
        }
        v.push(line);
        accumulator = accumulator + 1;
    }

    v
}

#[cfg(test)]
mod test_parse_solution {
    use super::*;
    use utils;

    #[test]
    fn make_distance_matrix_works() {
        // NODE_COORD_SECTION      (INDEX, X, Y):
        // 1	 1.0	 1.0
        // 2	 6.0	 1.0
        // 3	 1.0	 7.0
        // 4	 6.0	 7.0
        let mut nodes = Vec::default();
        nodes.push(parser::NodeCoordSection {
            index: 1,
            x: 1.0,
            y: 1.0,
        });
        nodes.push(parser::NodeCoordSection {
            index: 2,
            x: 6.0,
            y: 1.0,
        });
        nodes.push(parser::NodeCoordSection {
            index: 3,
            x: 1.0,
            y: 7.0,
        });
        nodes.push(parser::NodeCoordSection {
            index: 4,
            x: 6.0,
            y: 7.0,
        });

        //      1    2   3   4
        //      -    -   -   -
        // 1         5   6   8
        // 2             8   7
        // 3                 5
        // 4

        let mut v: Vec<Vec<u32>> = Vec::default();

        v.push([5, 6, 8].to_vec());
        v.push([8, 7].to_vec());
        v.push([5].to_vec());

        v.iter()
            .zip(make_distance_matrix(&nodes))
            .all(|(a, b)| utils::vec_compare(a, &b));
    }
}
