use common::puzzle::PuzzlePart;

use crate::PointList;

pub struct Puzzle09b {}

impl PuzzlePart for Puzzle09b {
    fn description() -> &'static str {
        "Find the area of the largest rectangle that can be formed within a filled-in section of tiles."
    }

    fn solve(input: &str) -> String {
        let points = PointList::parse(input);
        // dbg!(points);
        points.max_area_restricted().to_string()
    }
}
