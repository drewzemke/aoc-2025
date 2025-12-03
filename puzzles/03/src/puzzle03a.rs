use common::puzzle::PuzzlePart;

use crate::BatteryBank;

pub struct Puzzle03a {}

impl PuzzlePart for Puzzle03a {
    fn description() -> &'static str {
        "Compute sum of 2-digit 'joltages' from a bank of batteries."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(BatteryBank::parse)
            .map(|b| b.max_joltage(2))
            .sum::<u64>()
            .to_string()
    }
}
