use common::puzzle::PuzzlePart;

use crate::Diagram;

pub struct Puzzle07b {}

impl PuzzlePart for Puzzle07b {
    fn description() -> &'static str {
        "Count the number of timelines of a time-splitting quantum beam as it passes through a device."
    }

    fn solve(input: &str) -> String {
        Diagram::parse(input).count_beam_timelines().to_string()
    }
}
