use common::puzzle::PuzzlePart;

use crate::{DialRotation, Dir};

pub struct Puzzle01b {}

impl PuzzlePart for Puzzle01b {
    fn description() -> &'static str {
        "Find the 'password' again, but where passing 0 counts as hitting it."
    }

    fn solve(input: &str) -> String {
        let mut position = 50;
        let mut zero_hits = 0;

        input.lines().map(DialRotation::parse).for_each(|rot| {
            let sign = match rot.dir {
                Dir::Right => 1,
                Dir::Left => -1,
            };

            let new_pos = position + sign * rot.amount;

            // 2x2x2 cases: right/left, start on/off zero, end on/off zero
            let start_on_zero = position == 0;
            let end_on_zero = new_pos % 100 == 0;

            // yay brute force (in that I figured out this logic by just
            // writing tests and conrrecting values, rather than actually
            // thinking it through)
            let div = new_pos.div_euclid(100).abs();
            zero_hits += match (rot.dir, start_on_zero, end_on_zero) {
                (Dir::Right, true, true) => div,
                (Dir::Right, true, false) => div,
                (Dir::Right, false, true) => div,
                (Dir::Right, false, false) => div,
                (Dir::Left, true, true) => div,
                (Dir::Left, false, false) => div,
                (Dir::Left, true, false) => div - 1,
                (Dir::Left, false, true) => div + 1,
            };

            position = new_pos.rem_euclid(100);
        });

        zero_hits.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_zero_to_on_right() {
        let input = "R150";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "2");
    }

    #[test]
    fn off_zero_to_on_left() {
        let input = "L150";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "2");
    }

    #[test]
    fn off_zero_to_off_right() {
        let input = "R200";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "2");
    }

    #[test]
    fn off_zero_to_off_left() {
        let input = "L200";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "2");
    }

    #[test]
    fn on_zero_to_on_right() {
        let input = "L50\nR200";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "3");
    }

    #[test]
    fn on_zero_to_on_left() {
        let input = "L50\nL200";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "3");
    }

    #[test]
    fn on_zero_to_off_right() {
        let input = "L50\nR150";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "2");
    }

    #[test]
    fn on_zero_to_off_left() {
        let input = "L50\nL150";
        let output = Puzzle01b::solve(input);
        assert_eq!(&output, "2");
    }
}
