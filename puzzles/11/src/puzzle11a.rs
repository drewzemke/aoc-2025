use common::puzzle::PuzzlePart;

use crate::Graph;

pub struct Puzzle11a {}

impl PuzzlePart for Puzzle11a {
    fn description() -> &'static str {
        "Find the number of distinct paths from one node to another in a network."
    }

    fn solve(input: &str) -> String {
        let graph = Graph::parse(input);

        let start = graph.find_node("you");
        let end = graph.find_node("out");

        graph.count_paths(start, end).to_string()
    }
}
