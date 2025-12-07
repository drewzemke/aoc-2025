use common::grid_def;

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
}
