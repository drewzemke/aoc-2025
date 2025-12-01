use common::puzzle::PuzzlePart;

use crate::DialRotation;

pub struct Puzzle01a {}

impl PuzzlePart for Puzzle01a {
    fn description() -> &'static str {
        "Find the 'password' based on the rotation of a dial."
    }

    fn solve(input: &str) -> String {
        let mut position = 50;
        let mut zero_hits = 0;
        input.lines().map(DialRotation::parse).for_each(|rot| {
            let sign = match rot.dir {
                crate::Dir::Right => 1,
                crate::Dir::Left => -1,
            };

            position = (position + sign * rot.amount).rem_euclid(100);

            if position == 0 {
                zero_hits += 1;
            }
        });

        zero_hits.to_string()
    }
}
