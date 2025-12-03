use common::puzzle::PuzzlePart;

use crate::{parse, sum_invalid_ids_b};

pub struct Puzzle02b {}

impl PuzzlePart for Puzzle02b {
    fn description() -> &'static str {
        "Find the sum of invalid ids with relaxed criteria."
    }

    fn solve(input: &str) -> String {
        parse(input)
            .into_iter()
            .map(sum_invalid_ids_b)
            .sum::<u64>()
            .to_string()
    }
}
