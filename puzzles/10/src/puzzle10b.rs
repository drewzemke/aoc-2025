use common::puzzle::PuzzlePart;

use crate::Machine;

pub struct Puzzle10b {}

impl PuzzlePart for Puzzle10b {
    fn description() -> &'static str {
        "Find the minimum number of button presses needed to reach specific joltages on a machine."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(Machine::parse)
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .map(|(idx, m)| {
                println!("LINE {}", idx + 1);
                m.min_z_soln()
            })
            .map(|v| v.into_iter().sum::<usize>())
            .sum::<usize>()
            .to_string()
    }
}
