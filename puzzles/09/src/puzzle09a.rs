use common::puzzle::PuzzlePart;

use crate::PointList;

pub struct Puzzle09a {}

impl PuzzlePart for Puzzle09a {
    fn description() -> &'static str {
        "Find the area of the largest rectangle that can be formed with pairs of points in corners."
    }

    fn solve(input: &str) -> String {
        PointList::parse(input).max_area().to_string()
    }
}
