use common::puzzle::PuzzlePart;

use crate::Diagram;

pub struct Puzzle07a {}

impl PuzzlePart for Puzzle07a {
    fn description() -> &'static str {
        "Count the number of times a beam is split as it passes through a device."
    }

    fn solve(input: &str) -> String {
        Diagram::parse(input).count_beam_splits().to_string()
    }
}
