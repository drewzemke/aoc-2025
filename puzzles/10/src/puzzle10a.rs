use common::puzzle::PuzzlePart;

use crate::Machine;

pub struct Puzzle10a {}

impl PuzzlePart for Puzzle10a {
    fn description() -> &'static str {
        "Find the minimum number of button presses needed to turn on lights on a machine."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(Machine::parse)
            .collect::<Vec<_>>()
            .iter()
            .map(Machine::min_z2_soln)
            .map(|v| v.into_iter().filter(|b| *b).count())
            .sum::<usize>()
            .to_string()
    }
}
