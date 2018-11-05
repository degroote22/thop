#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
mod change_item;
mod evaluate;
mod greedy;
mod inputs;
mod insert_city;
mod instance;
mod local_search;
mod makers;
mod parser;
mod printer;
mod sa;
mod switch_cities;
mod utils;
mod vns;

fn main() {
    // só pro compilador não falar que tem coisa não sendo usada
    if false {
        printer::print_results_part_0();
    } else {
        printer::print_results_part_1();
    }
}
