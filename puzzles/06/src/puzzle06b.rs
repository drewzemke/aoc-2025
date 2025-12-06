use common::puzzle::PuzzlePart;

use crate::Worksheet;

pub struct Puzzle06b {}

impl PuzzlePart for Puzzle06b {
    fn description() -> &'static str {
        "Find the sum of performing a bunch of math ops on columns of *vertical* numbers."
    }

    fn solve(input: &str) -> String {
        Worksheet::parse(input)
            .0
            .iter()
            .map(|col| col.cephalopod_value())
            .sum::<i64>()
            .to_string()
    }
}
