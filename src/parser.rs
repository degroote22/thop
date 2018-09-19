use makers;
use regex::Regex;
use std::collections::HashMap;
use utils;

lazy_static! {
    static ref RE: Regex = Regex::new("\\s").unwrap();
}

#[derive(Debug)]
pub enum KnapsackDataType {
    Uncorrelated,
    BoundedStronglyCorrelated,
    UncorrelatedSimilarWeights,
}

#[derive(Debug)]
pub enum EdgeWeightType {
    Ceil2d,
}

#[derive(Debug, PartialEq)]
pub struct NodeCoordSection {
    pub index: u32,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq)]
pub struct ItemSection {
    pub index: u32,
    pub profit: u32,
    pub weight: u32,
    pub assigned_city_id: u32,
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

pub struct SuperFile<'a> {
    instance: &'a THOPFile,
    distance_matrix: Vec<Vec<u32>>,
    items_hash: HashMap<u32, &'a ItemSection>,
}
impl<'a> SuperFile<'a> {
    pub fn new(instance: &'a THOPFile) -> SuperFile {
        SuperFile {
            instance,
            distance_matrix: makers::make_distance_matrix(&instance.node_coord_section),
            items_hash: makers::make_hash_map(&instance.items_section),
        }
    }

    pub fn get_max_speed(&self) -> f64 {
        self.instance.max_speed.unwrap()
    }

    pub fn get_item_details(&self, asked: u32) -> &ItemSection {
        self.items_hash.get(&asked).unwrap()
    }

    pub fn visit_city(&self, city: u32, items: &Vec<u32>) -> (u32, u32, u32) {
        // (u32: weight, u32: profit, u32: n items catched)
        let mut weight = 0;
        let mut profit = 0;
        let mut caught = 0;

        for asked in items {
            let item_details = self.get_item_details(*asked);

            if item_details.assigned_city_id == city {
                caught += 1;
                weight += item_details.weight;
                profit += item_details.profit;
            }
        }
        (weight, profit, caught)
    }

    pub fn speed_descresc_per_weight(&self) -> f64 {
        (self.instance.max_speed.unwrap() - self.instance.min_speed.unwrap())
            / (self.instance.capacity_of_knapsack.unwrap() as f64)
    }

    pub fn get_capacity_of_knapsack(&self) -> u32 {
        self.instance.capacity_of_knapsack.unwrap()
    }

    pub fn get_max_time(&self) -> u32 {
        self.instance.max_time.unwrap()
    }

    pub fn get_distance(&self, a: &u32, b: &u32) -> u32 {
        if a == b {
            return 0;
        };
        // o menor valor é a linha
        // menos um por causa do index
        // --
        // a coluna é o maior valor menos a linha
        // menos um por causa do index
        //      1    2   3   4
        //      -    -   -   -
        // 1         5   6   8
        // 2             8   6
        // 3                 5
        // 4

        let min = a.min(b);
        let max = a.max(b);

        let line = (*min - 1) as usize;
        let row = (*max - min - 1) as usize;
        *self
            .distance_matrix
            .get(line)
            .expect("unable to get line")
            .get(row)
            .expect("unable to get row")
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

        let instance = parse_instance(&contents);
        let super_file = SuperFile::new(&instance);

        assert_eq!(super_file.get_distance(&1, &1), 0);
        assert_eq!(super_file.get_distance(&1, &2), 5);
        assert_eq!(super_file.get_distance(&2, &1), 5);
        assert_eq!(super_file.get_distance(&1, &3), 6);
        assert_eq!(super_file.get_distance(&3, &1), 6);
        assert_eq!(super_file.get_distance(&1, &4), 8);
        assert_eq!(super_file.get_distance(&4, &1), 8);
        assert_eq!(super_file.get_distance(&2, &3), 8);
        assert_eq!(super_file.get_distance(&3, &2), 8);
        assert_eq!(super_file.get_distance(&2, &4), 6);
        assert_eq!(super_file.get_distance(&4, &2), 6);
        assert_eq!(super_file.get_distance(&3, &4), 5);
        assert_eq!(super_file.get_distance(&4, &3), 5);

        assert_eq!(super_file.visit_city(2, &[1, 2].to_vec()), (5, 50, 2));
        assert_eq!(super_file.visit_city(3, &[3, 4, 5].to_vec()), (5, 180, 3));

        assert_eq!(super_file.speed_descresc_per_weight(), 0.3);
    }
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
    match RE.replace_all(left_side, "").to_string().as_ref() {
        "PROBLEMNAME" => {
            parsed_file.problem_name = Some(right_side.to_string());
        }
        "KNAPSACKDATATYPE" => parse_knapsack_data_type(right_side, parsed_file),
        "DIMENSION" => {
            parsed_file.dimension = Some(right_side.parse::<u32>().unwrap());
        }
        "NUMBEROFITEMS" => {
            parsed_file.number_of_items = Some(right_side.parse::<u32>().unwrap());
        }
        "CAPACITYOFKNAPSACK" => {
            parsed_file.capacity_of_knapsack = Some(right_side.parse::<u32>().unwrap());
        }
        "MAXTIME" => {
            parsed_file.max_time = Some(right_side.parse::<u32>().unwrap());
        }
        "MINSPEED" => {
            parsed_file.min_speed = Some(right_side.parse::<f64>().unwrap());
        }
        "MAXSPEED" => {
            parsed_file.max_speed = Some(right_side.parse::<f64>().unwrap());
        }
        "EDGE_WEIGHT_TYPE" => parse_edge_weight_type(right_side, parsed_file),
        "NODE_COORD_SECTION(INDEX,X,Y)" => {
            return ParsingStage::ParseCoords;
        }
        // no tipo do input-a é assim
        "ITEMSSECTION(INDEX,PROFIT,WEIGHT,ASSIGNED_CITY_ID)" => {
            return ParsingStage::ParseItems;
        }
        // no tipo do input-b é assim
        "ITEMSSECTION(INDEX,PROFIT,WEIGHT,ASSIGNEDNODENUMBER)" => {
            return ParsingStage::ParseItems;
        }
        _ => panic!("Line not recognized {}", left_side),
    }
    return ParsingStage::Default;
}

fn parse_coords(line: &str) -> NodeCoordSection {
    let nums: Vec<&str> = line.split(" ").collect();
    let _index = nums.get(0).unwrap();
    let _x = nums.get(1).unwrap();
    let _y = nums.get(2).unwrap();
    let index = RE.replace_all(_index, "").to_string();
    let x = RE.replace_all(_x, "").to_string();
    let y = RE.replace_all(_y, "").to_string();
    NodeCoordSection {
        index: index.parse::<u32>().unwrap(),
        x: x.parse::<f64>().unwrap(),
        y: y.parse::<f64>().unwrap(),
    }
}

fn parse_items(line: &str) -> ItemSection {
    // no tipo do input a é assim
    let mut nums: Vec<&str> = line.split("\t").collect();

    if nums.len() == 1 {
        // no tipo do input-b é assim
        nums = line.split(" ").collect();
    }

    let _index = nums.get(0).unwrap();
    let _profit = nums.get(1).unwrap();
    let _weight = nums.get(2).unwrap();
    let _assigned_city_id = nums.get(3).unwrap();
    let index = RE.replace_all(_index, "").to_string();
    let profit = RE.replace_all(_profit, "").to_string();
    let weight = RE.replace_all(_weight, "").to_string();
    let assigned_city_id = RE.replace_all(_assigned_city_id, "").to_string();
    ItemSection {
        index: index.parse::<u32>().unwrap(),
        profit: profit.parse::<u32>().unwrap(),
        weight: weight.parse::<u32>().unwrap(),
        assigned_city_id: assigned_city_id.parse::<u32>().unwrap(),
    }
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
        node_coord_section: Vec::default(),
        items_section: Vec::default(),
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
                ParsingStage::ParseCoords => {
                    parsed_file.node_coord_section.push(parse_coords(line))
                }
                ParsingStage::ParseItems => parsed_file.items_section.push(parse_items(line)),
            }
        }
    }
    return parsed_file;
}

#[cfg(test)]
mod test_parse_instance {
    use super::*;

    #[test]
    fn parse_coords_works() {
        assert_eq!(
            parse_coords("1	 1.0	 1.0"),
            NodeCoordSection {
                index: 1,
                x: 1.0,
                y: 1.0,
            },
        );
    }

    #[test]
    fn parse_items_works() {
        assert_eq!(
            parse_items("1	 20	2	2"),
            ItemSection {
                index: 1,
                profit: 20,
                weight: 2,
                assigned_city_id: 2
            }
        )
    }
}

#[derive(Debug)]
pub struct SolutionFile {
    pub route: Vec<u32>,
    pub items: Vec<u32>,
}

impl PartialEq for SolutionFile {
    fn eq(&self, other: &SolutionFile) -> bool {
        utils::vec_compare(&self.route, &other.route)
            && utils::vec_compare(&self.items, &other.items)
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
mod test_parse_solution {
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
