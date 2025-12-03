use common::puzzle::PuzzlePart;

use crate::BatteryBank;

pub struct Puzzle03a {}

impl PuzzlePart for Puzzle03a {
    fn description() -> &'static str {
        "Compute the maximum total 'joltage' from a bank of batteries."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(BatteryBank::parse)
            .map(|b| b.max_joltage())
            .sum::<u32>()
            .to_string()
    }
}
