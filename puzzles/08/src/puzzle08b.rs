use common::puzzle::PuzzlePart;

use crate::JunctionBox;

pub struct Puzzle08b {}

impl PuzzlePart for Puzzle08b {
    fn description() -> &'static str {
        "Find the product of x-coords of the last connections required to join all of the circuit boxes."
    }

    fn solve(input: &str) -> String {
        let boxes: Vec<_> = input.lines().map(JunctionBox::parse).collect();

        let (pt1, pt2) = JunctionBox::last_connection(&boxes);
        (pt1.0 * pt2.0).to_string()
    }
}
