use common::puzzle::PuzzlePart;

use crate::PaperRollGrid;

pub struct Puzzle04b {}

impl PuzzlePart for Puzzle04b {
    fn description() -> &'static str {
        "Find the number of rolls of paper that can be (iteratively) removed."
    }

    fn solve(input: &str) -> String {
        let mut grid = PaperRollGrid::parse(input);

        let mut total = 0;

        loop {
            let removed = grid.remove_accessible_rolls();
            if removed == 0 {
                break;
            }
            total += removed;
        }

        total.to_string()
    }
}
