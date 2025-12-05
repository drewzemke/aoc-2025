use common::puzzle::PuzzlePart;

use crate::IngredientDb;

pub struct Puzzle05a {}

impl PuzzlePart for Puzzle05a {
    fn description() -> &'static str {
        "Count the number of fresh ingredients in an ingredients list."
    }

    fn solve(input: &str) -> String {
        let mut db = IngredientDb::parse(input);
        db.merge_ranges();

        let IngredientDb {
            ranges,
            ingredients,
        } = db;

        ingredients
            .into_iter()
            .filter(|v| ranges.iter().any(|range| range.contains(v)))
            .count()
            .to_string()
    }
}
