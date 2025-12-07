use std::collections::HashMap;

use common::{grid_def, point::Point};

pub mod puzzle07a;
pub mod puzzle07b;

#[derive(Debug, Clone)]
pub enum Tile {
    Start,
    Splitter,
    Nothing,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '^' => Self::Splitter,
            '.' => Self::Nothing,
            _ => unreachable!(),
        }
    }
}

grid_def!(Diagram, Tile);

impl Diagram {
    pub fn count_beam_splits(&self) -> usize {
        let mut splits = 0;

        let mut beam_positions = vec![false; self.width()];

        let start_pos = self.find_pt(|t| matches!(t, Tile::Start)).unwrap();
        beam_positions[start_pos.col as usize] = true;

        // skipping the first row (which has the start tile),
        // iteratively compute the beams in the next row from the previous row
        for row in self.rows().skip(1) {
            for (idx, tile) in row.enumerate() {
                if matches!(tile, Tile::Splitter) && beam_positions[idx] {
                    splits += 1;
                    beam_positions[idx] = false;

                    if idx > 0 {
                        beam_positions[idx - 1] = true;
                    }

                    if idx < self.width() - 1 {
                        beam_positions[idx + 1] = true;
                    }
                }
            }
        }

        splits
    }

    pub fn count_beam_timelines(&self) -> usize {
        let start_pos = self.find_pt(|t| matches!(t, Tile::Start)).unwrap();
        let mut memo = HashMap::new();

        self.count_timelines_from(start_pos, &mut memo)
    }

    fn count_timelines_from(&self, pt: Point, memo: &mut HashMap<Point, usize>) -> usize {
        // check the memo
        if let Some(val) = memo.get(&pt) {
            return *val;
        }

        // base case: we're at the bottom and can't split further
        if (pt.row as usize) >= self.height() - 2 {
            return 1;
        }

        // look two rows down to see if we'll split
        let val = if let Some(tile) = self.at((pt.row + 2, pt.col).into())
            && matches!(tile, Tile::Splitter)
        {
            // split there into left and right, compute the number of paths for each
            // of those, then add them together
            let left = self.count_timelines_from((pt.row + 2, pt.col - 1).into(), memo);
            let right = self.count_timelines_from((pt.row + 2, pt.col + 1).into(), memo);

            left + right
        } else {
            // if there's no splitter, continue checking downwards
            self.count_timelines_from((pt.row + 2, pt.col).into(), memo)
        };

        memo.insert(pt, val);
        val
    }
}
