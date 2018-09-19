use super::*;

enum ParsingStage {
    Default,
    ParseCoords,
    ParseItems,
}

fn parse_knapsack_data_type(right_side: &str, parsed_file: &mut THOPFile) {
    match right_side.as_ref() {
        "uncorrelated" => {
            parsed_file.knapsack_data_type = Some(KnapsackDataType::Uncorrelated);
        }
        "bounded-strongly-correlated" => {
            parsed_file.knapsack_data_type = Some(KnapsackDataType::BoundedStronglyCorrelated);
        }
        "boundedstronglycorr" => {
            parsed_file.knapsack_data_type = Some(KnapsackDataType::BoundedStronglyCorrelated);
        }
        "uncorrelated-similar-weights" => {
            parsed_file.knapsack_data_type = Some(KnapsackDataType::UncorrelatedSimilarWeights);
        }
        "uncorrelated,similarweights" => {
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

pub fn parse(contents: &str) -> THOPFile {
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
                ParsingStage::ParseCoords => {
                    parsed_file.node_coord_section.push(parse_coords(line))
                }
                ParsingStage::ParseItems => parsed_file.items_section.push(parse_items(line)),
                _ => panic!("Wrong stage when parsing. Expected to be capturing block."),
            }
        }
    }
    parsed_file
}
