use common::puzzle::PuzzlePart;

use crate::JunctionBox;

pub struct Puzzle08a {}

impl PuzzlePart for Puzzle08a {
    fn description() -> &'static str {
        "Find the product of the sizes of the three largest circuits in a 3D arrangment of circuit boxes."
    }

    fn solve_with_context(input: &str, is_example: bool) -> String {
        let conns = if is_example { 10 } else { 1000 };

        let boxes: Vec<_> = input.lines().map(JunctionBox::parse).collect();

        let mut circuits = JunctionBox::make_circuits(&boxes, conns);
        circuits.sort_unstable_by_key(|v| std::cmp::Reverse(v.len()));

        (circuits[0].len() * circuits[1].len() * circuits[2].len()).to_string()
    }
}
