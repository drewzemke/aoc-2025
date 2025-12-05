use common::puzzle::PuzzlePart;

use crate::IngredientDb;

pub struct Puzzle05b {}

impl PuzzlePart for Puzzle05b {
    fn description() -> &'static str {
        "Count the total number of fresh ingredients in an ingredients."
    }

    fn solve(input: &str) -> String {
        let mut db = IngredientDb::parse(input);
        db.merge_ranges();

        db.ranges
            .into_iter()
            .map(|range| range.end() - range.start() + 1)
            .sum::<u64>()
            .to_string()
    }
}
