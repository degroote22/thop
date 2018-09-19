use super::SolutionFile;
use utils;

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

pub fn parse(contents: &str) -> SolutionFile {
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
    use super::{parse, parse_int_list, SolutionFile};

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
            parse("[]\n[]"),
            SolutionFile {
                route: [].to_vec(),
                items: [].to_vec()
            }
        );
        assert_eq!(
            parse("[1]\n[1]"),
            SolutionFile {
                route: [1].to_vec(),
                items: [1].to_vec()
            }
        );
        assert_eq!(
            parse("[1,2]\n[1,2]"),
            SolutionFile {
                route: [1, 2].to_vec(),
                items: [1, 2].to_vec()
            }
        );
        assert_eq!(
            parse("[1,2,3]\n[1,2,3]"),
            SolutionFile {
                route: [1, 2, 3].to_vec(),
                items: [1, 2, 3].to_vec()
            }
        );
    }
}
