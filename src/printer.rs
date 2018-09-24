use evaluate;
use instance;
use parser;
use std::fs::File;
use std::io::prelude::*;

pub fn print_results_part_0() {
    for (i, s) in [
        // n5_1
        (
            "./input-a/instances/ex4-n5_1.thop",
            "./input-a/solutions/ex4-n5_a.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_1.thop",
            "./input-a/solutions/ex4-n5_b.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_1.thop",
            "./input-a/solutions/ex4-n5_c.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_1.thop",
            "./input-a/solutions/ex4-n5_d.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_1.thop",
            "./input-a/solutions/ex4-n5_e.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_1.thop",
            "./input-a/solutions/ex4-n5_f.thop.sol",
        ),
        // n5_2
        (
            "./input-a/instances/ex4-n5_2.thop",
            "./input-a/solutions/ex4-n5_a.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_2.thop",
            "./input-a/solutions/ex4-n5_d.thop.sol",
        ),
        // n5_3
        (
            "./input-a/instances/ex4-n5_3.thop",
            "./input-a/solutions/ex4-n5_a.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_3.thop",
            "./input-a/solutions/ex4-n5_b.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_3.thop",
            "./input-a/solutions/ex4-n5_c.thop.sol",
        ),
        (
            "./input-a/instances/ex4-n5_3.thop",
            "./input-a/solutions/ex4-n5_d.thop.sol",
        ),
        // eil51_n147_bounded-strongly-corr_01_
        (
            "./input-b/instances/eil51-thop/eil51_n147_bounded-strongly-corr_01_01.thop",
            "./input-b/solutions/eil51-thop/eil51_n147_bounded-strongly-corr_01_01.thop.sol",
        ),
        (
            "./input-b/instances/eil51-thop/eil51_n147_bounded-strongly-corr_01_02.thop",
            "./input-b/solutions/eil51-thop/eil51_n147_bounded-strongly-corr_01_02.thop.sol",
        ),
        (
            "./input-b/instances/eil51-thop/eil51_n147_bounded-strongly-corr_01_03.thop",
            "./input-b/solutions/eil51-thop/eil51_n147_bounded-strongly-corr_01_03.thop.sol",
        ),
        // eil51_n147_bounded-strongly-corr_05_
        (
            "./input-b/instances/eil51-thop/eil51_n147_bounded-strongly-corr_05_01.thop",
            "./input-b/solutions/eil51-thop/eil51_n147_bounded-strongly-corr_05_01.thop.sol",
        ),
        (
            "./input-b/instances/eil51-thop/eil51_n147_bounded-strongly-corr_05_02.thop",
            "./input-b/solutions/eil51-thop/eil51_n147_bounded-strongly-corr_05_02.thop.sol",
        ),
        (
            "./input-b/instances/eil51-thop/eil51_n147_bounded-strongly-corr_05_03.thop",
            "./input-b/solutions/eil51-thop/eil51_n147_bounded-strongly-corr_05_03.thop.sol",
        ),
        // eil51_n490_uncorr-similar-weights_05_
        (
            "./input-b/instances/eil51-thop/eil51_n490_uncorr-similar-weights_05_01.thop",
            "./input-b/solutions/eil51-thop/eil51_n490_uncorr-similar-weights_05_01.thop.sol",
        ),
        (
            "./input-b/instances/eil51-thop/eil51_n490_uncorr-similar-weights_05_02.thop",
            "./input-b/solutions/eil51-thop/eil51_n490_uncorr-similar-weights_05_02.thop.sol",
        ),
        (
            "./input-b/instances/eil51-thop/eil51_n490_uncorr-similar-weights_05_03.thop",
            "./input-b/solutions/eil51-thop/eil51_n490_uncorr-similar-weights_05_03.thop.sol",
        ),
        // a280_n278_bounded-strongly-corr_
        (
            "./input-b/instances/a280-thop/a280_n278_bounded-strongly-corr_01_01.thop",
            "./input-b/solutions/a280-thop/a280_n278_bounded-strongly-corr_01_01.thop.sol",
        ),
        (
            "./input-b/instances/a280-thop/a280_n278_bounded-strongly-corr_05_01.thop",
            "./input-b/solutions/a280-thop/a280_n278_bounded-strongly-corr_05_01.thop.sol",
        ),
        (
            "./input-b/instances/a280-thop/a280_n278_bounded-strongly-corr_10_01.thop",
            "./input-b/solutions/a280-thop/a280_n278_bounded-strongly-corr_10_01.thop.sol",
        ),
        // a280 misc
        (
            "./input-b/instances/a280-thop/a280_n2780_bounded-strongly-corr_10_03.thop",
            "./input-b/solutions/a280-thop/a280_n2780_bounded-strongly-corr_10_03.thop.sol",
        ),
        (
            "./input-b/instances/a280-thop/a280_n2780_uncorr_10_03.thop",
            "./input-b/solutions/a280-thop/a280_n2780_uncorr_10_03.thop.sol",
        ),
        (
            "./input-b/instances/a280-thop/a280_n2780_uncorr-similar-weights_10_03.thop",
            "./input-b/solutions/a280-thop/a280_n2780_uncorr-similar-weights_10_03.thop.sol",
        ),
        // dsj1000
        (
            "./input-b/instances/dsj1000-thop/dsj1000_n998_bounded-strongly-corr_01_01.thop",
            "./input-b/solutions/dsj1000-thop/dsj1000_n998_bounded-strongly-corr_01_01.thop.sol",
        ),
        (
            "./input-b/instances/dsj1000-thop/dsj1000_n998_uncorr_01_01.thop",
            "./input-b/solutions/dsj1000-thop/dsj1000_n998_uncorr_01_01.thop.sol",
        ),
        (
            "./input-b/instances/dsj1000-thop/dsj1000_n998_uncorr-similar-weights_01_01.thop",
            "./input-b/solutions/dsj1000-thop/dsj1000_n998_uncorr-similar-weights_01_01.thop.sol",
        ),
    ]
        .iter()
    {
        let mut f = File::open(i).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let mut f2 = File::open(s).expect("file not found");
        let mut contents2 = String::new();
        f2.read_to_string(&mut contents2)
            .expect("something went wrong reading the file");

        let solution = parser::solution::parse(&contents2);
        let instance_file = parser::instance::parse(&contents);
        let instance = instance::Instance::new(&instance_file);

        let result = evaluate::Evaluator::new(&instance).calc(&solution);

        println!("Evaluating {} {}", i, s);
        println!("Profit: {}", result.profit);
        println!("Time: {}", result.time);
        println!("Okay: {}", result.okay);
        println!("");
    }
}
