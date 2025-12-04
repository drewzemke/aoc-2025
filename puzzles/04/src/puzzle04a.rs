use common::puzzle::PuzzlePart;

use crate::PaperRollGrid;

pub struct Puzzle04a {}

impl PuzzlePart for Puzzle04a {
    fn description() -> &'static str {
        "Find the number of rolls of paper in a grid with fewer than four occupied neighbors."
    }

    fn solve(input: &str) -> String {
        PaperRollGrid::parse(input)
            .accessible_rolls()
            .rows()
            .flatten()
            .filter(|is_accessible| *is_accessible)
            .count()
            .to_string()
    }
}
