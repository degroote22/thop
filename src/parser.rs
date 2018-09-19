use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new("\\s").unwrap();
}

#[derive(Debug)]
enum KnapsackDataType {
    Uncorrelated,
    BoundedStronglyCorrelated,
    UncorrelatedSimilarWeights,
}

#[derive(Debug)]
enum EdgeWeightType {
    Ceil2d,
}

#[derive(Debug, Clone)]
struct NodeCoordSection {
    index: u32,
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct ItemSection {
    index: u32,
    profit: u32,
    weight: u32,
    assigned_city_id: u32,
}

#[derive(Debug)]
pub struct THOPFile {
    problem_name: Option<String>,
    knapsack_data_type: Option<KnapsackDataType>,
    dimension: Option<u32>,
    number_of_items: Option<u32>,
    capacity_of_knapsack: Option<u32>,
    max_time: Option<u32>,
    min_speed: Option<f64>,
    max_speed: Option<f64>,
    edge_weight_type: Option<EdgeWeightType>,
    node_coord_section: Vec<NodeCoordSection>,
    items_section: Vec<ItemSection>,
}

fn parse_knapsack_data_type(right_side: &str, parsed_file: &mut THOPFile) {
    match right_side.as_ref() {
        "uncorrelated" => {
            parsed_file.knapsack_data_type = Some(KnapsackDataType::Uncorrelated);
        }
        "bounded-strongly-correlated" => {
            parsed_file.knapsack_data_type = Some(KnapsackDataType::BoundedStronglyCorrelated);
        }
        "uncorrelated-similar-weights" => {
            parsed_file.knapsack_data_type = Some(KnapsackDataType::UncorrelatedSimilarWeights);
        }
        _ => panic!("Should have a Knapsack Data Type, got: {}", right_side),
    }
}
fn parse_edge_weight_type(right_side: &str, parsed_file: &mut THOPFile) {
    match right_side.as_ref() {
        "CEIL_2D" => {
            parsed_file.edge_weight_type = Some(EdgeWeightType::Ceil2d);
        }
        _ => panic!("Should have a EDGE_WEIGHT_TYPE, got: {}", right_side),
    }
}

enum ParsingStage {
    Default,
    ParseCoords,
    ParseItems,
}

fn parse_field(left_side: &str, right_side: &str, parsed_file: &mut THOPFile) -> ParsingStage {
    match left_side.as_ref() {
        "PROBLEM NAME" => {
            parsed_file.problem_name = Some(right_side.to_string());
        }
        "KNAPSACK DATA TYPE" => parse_knapsack_data_type(right_side, parsed_file),
        "DIMENSION" => {
            parsed_file.dimension = Some(right_side.parse::<u32>().unwrap());
        }
        "NUMBER OF ITEMS" => {
            parsed_file.number_of_items = Some(right_side.parse::<u32>().unwrap());
        }
        "CAPACITY OF KNAPSACK" => {
            parsed_file.capacity_of_knapsack = Some(right_side.parse::<u32>().unwrap());
        }
        "MAX TIME" => {
            parsed_file.max_time = Some(right_side.parse::<u32>().unwrap());
        }
        "MIN SPEED" => {
            parsed_file.min_speed = Some(right_side.parse::<f64>().unwrap());
        }
        "MAX SPEED" => {
            parsed_file.max_speed = Some(right_side.parse::<f64>().unwrap());
        }
        "EDGE_WEIGHT_TYPE" => parse_edge_weight_type(right_side, parsed_file),
        "NODE_COORD_SECTION      (INDEX, X, Y)" => {
            return ParsingStage::ParseCoords;
        }
        "ITEMS SECTION	(INDEX, PROFIT, WEIGHT, ASSIGNED_CITY_ID)" => {
            return ParsingStage::ParseItems;
        }
        _ => panic!("Line not recognized {}", left_side),
    }
    return ParsingStage::Default;
}

fn parse_coords(line: &str, parsed_file: &mut THOPFile) {
    let nums: Vec<&str> = line.split(" ").collect();
    let _index = nums.get(0).unwrap();
    let _x = nums.get(1).unwrap();
    let _y = nums.get(2).unwrap();
    let index = RE.replace_all(_index, "").to_string();
    let x = RE.replace_all(_x, "").to_string();
    let y = RE.replace_all(_y, "").to_string();
    parsed_file.node_coord_section.push(NodeCoordSection {
        index: index.parse::<u32>().unwrap(),
        x: x.parse::<f64>().unwrap(),
        y: y.parse::<f64>().unwrap(),
    })
}

fn parse_items(line: &str, parsed_file: &mut THOPFile) {
    let nums: Vec<&str> = line.split("\t").collect();
    let _index = nums.get(0).unwrap();
    let _profit = nums.get(1).unwrap();
    let _weight = nums.get(2).unwrap();
    let _assigned_city_id = nums.get(3).unwrap();
    let index = RE.replace_all(_index, "").to_string();
    let profit = RE.replace_all(_profit, "").to_string();
    let weight = RE.replace_all(_weight, "").to_string();
    let assigned_city_id = RE.replace_all(_assigned_city_id, "").to_string();
    parsed_file.items_section.push(ItemSection {
        index: index.parse::<u32>().unwrap(),
        profit: profit.parse::<u32>().unwrap(),
        weight: weight.parse::<u32>().unwrap(),
        assigned_city_id: assigned_city_id.parse::<u32>().unwrap(),
    })
}

pub fn parse_instance(contents: &str) -> THOPFile {
    let mut parsed_file = THOPFile {
        problem_name: None,
        knapsack_data_type: None,
        dimension: None,
        number_of_items: None,
        capacity_of_knapsack: None,
        max_time: None,
        min_speed: None,
        max_speed: None,
        edge_weight_type: None,
        node_coord_section: [].to_vec(),
        items_section: [].to_vec(),
    };

    let lines = contents.lines();
    let mut stage = ParsingStage::Default;
    for line in lines {
        let sides: Vec<&str> = line.split(":").collect();

        if sides.len() > 1 {
            let left_side = sides.get(0).unwrap();
            let _right_side = sides.get(1).unwrap();
            // remove espaços em branco
            let right_side = RE.replace_all(_right_side, "").to_string();

            // parse_field retorna o próximo estágio do parser
            // que pode ir para a fase de capturar blocos (abaixo)
            stage = parse_field(left_side, &right_side, &mut parsed_file);
        } else {
            match stage {
                ParsingStage::Default => {
                    panic!("Wrong stage when parsing. Expected to be capturing block.")
                }
                ParsingStage::ParseCoords => parse_coords(line, &mut parsed_file),
                ParsingStage::ParseItems => parse_items(line, &mut parsed_file),
            }
        }
    }
    return parsed_file;
}

#[derive(Debug)]
pub struct SolutionFile {
    pub route: Vec<u32>,
    pub items: Vec<u32>,
}

fn vec_compare(va: &[u32], vb: &[u32]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter() // iterate
       .zip(vb) // zipa
       .all(|(a,b)| a==b) // compara
}

impl PartialEq for SolutionFile {
    fn eq(&self, other: &SolutionFile) -> bool {
        vec_compare(&self.route, &other.route) && vec_compare(&self.items, &other.items)
    }
}

fn parse_int_list(source: &str) -> Vec<u32> {
    let end = source.len() - 1;
    // remove o [ do começo e o ] do final
    let substring = &source[1..end];

    if substring.len() == 0 {
        return [].to_vec();
    }

    let nums: Vec<&str> = substring.split(",").collect();
    nums.iter().map(|&x| x.parse::<u32>().unwrap()).collect()
}

pub fn parse_solution(contents: &str) -> SolutionFile {
    let mut parsed_solution = SolutionFile {
        route: [].to_vec(),
        items: [].to_vec(),
    };

    let mut lines = contents.lines();
    parsed_solution.route = parse_int_list(&lines.next().unwrap().to_string());
    parsed_solution.items = parse_int_list(&lines.next().unwrap().to_string());

    return parsed_solution;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_list_works() {
        assert_eq!(parse_int_list("[1,2,3]"), [1, 2, 3]);
        assert_eq!(parse_int_list("[1,2]"), [1, 2]);
        assert_eq!(parse_int_list("[1]"), [1]);
        assert_eq!(parse_int_list("[]"), []);
    }

    #[test]
    fn parse_solution_works() {
        assert_eq!(
            parse_solution("[]\n[]"),
            SolutionFile {
                route: [].to_vec(),
                items: [].to_vec()
            }
        );
        assert_eq!(
            parse_solution("[1]\n[1]"),
            SolutionFile {
                route: [1].to_vec(),
                items: [1].to_vec()
            }
        );
        assert_eq!(
            parse_solution("[1,2]\n[1,2]"),
            SolutionFile {
                route: [1, 2].to_vec(),
                items: [1, 2].to_vec()
            }
        );
        assert_eq!(
            parse_solution("[1,2,3]\n[1,2,3]"),
            SolutionFile {
                route: [1, 2, 3].to_vec(),
                items: [1, 2, 3].to_vec()
            }
        );
    }
}
