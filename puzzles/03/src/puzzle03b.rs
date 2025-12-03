use common::puzzle::PuzzlePart;

use crate::BatteryBank;

pub struct Puzzle03b {}

impl PuzzlePart for Puzzle03b {
    fn description() -> &'static str {
        "Compute sum of 12-digit 'joltages' from a bank of batteries."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(BatteryBank::parse)
            .map(|b| b.max_joltage(12))
            .sum::<u64>()
            .to_string()
    }
}
