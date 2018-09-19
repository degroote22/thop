use regex::Regex;
pub mod instance;
pub mod solution;
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
    pub problem_name: Option<String>,
    pub knapsack_data_type: Option<KnapsackDataType>,
    pub dimension: Option<u32>,
    pub number_of_items: Option<u32>,
    pub capacity_of_knapsack: Option<u32>,
    pub max_time: Option<u32>,
    pub min_speed: Option<f64>,
    pub max_speed: Option<f64>,
    pub edge_weight_type: Option<EdgeWeightType>,
    pub node_coord_section: Vec<NodeCoordSection>,
    pub items_section: Vec<ItemSection>,
}

#[derive(Debug)]
pub struct SolutionFile {
    pub route: Vec<u32>,
    pub items: Vec<u32>,
}
