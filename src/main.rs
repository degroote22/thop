#[macro_use]
extern crate lazy_static;
extern crate regex;
mod evaluate;
mod greedy;
mod inputs;
mod instance;
mod makers;
mod parser;
mod printer;
mod utils;

fn main() {
    // só pro compilador não falar que tem coisa não sendo usada
    if false {
        printer::print_results_part_0();
    } else {
        printer::print_results_part_1();
    }
}
