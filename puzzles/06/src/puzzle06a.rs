use common::puzzle::PuzzlePart;

use crate::Worksheet;

pub struct Puzzle06a {}

impl PuzzlePart for Puzzle06a {
    fn description() -> &'static str {
        "Find the sum of performing a bunch of math ops on columns of numbers."
    }

    fn solve(input: &str) -> String {
        Worksheet::parse(input)
            .0
            .iter()
            .map(|col| col.value())
            .sum::<i64>()
            .to_string()
    }
}
