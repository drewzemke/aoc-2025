use common::puzzle::PuzzlePart;

use crate::{parse, sum_invalid_ids_a};

pub struct Puzzle02a {}

impl PuzzlePart for Puzzle02a {
    fn description() -> &'static str {
        "Find the sum of invalid ids in rangers of numbers."
    }

    fn solve(input: &str) -> String {
        parse(input)
            .into_iter()
            .map(sum_invalid_ids_a)
            .sum::<u64>()
            .to_string()
    }
}
