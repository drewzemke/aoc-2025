use common::{dir::Dir8, grid::Grid, grid_def, point::Point};

pub mod puzzle04a;
pub mod puzzle04b;

#[derive(Clone, Debug)]
pub enum Tile {
    Roll,
    Nothing,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '@' => Self::Roll,
            '.' => Self::Nothing,
            _ => unreachable!(),
        }
    }
}

grid_def!(PaperRollGrid, Tile);

impl PaperRollGrid {
    // returns a grid of flags indicating which roll have fewer than 4 rolls as neighbors
    pub fn accessible_rolls(&self) -> Grid<bool> {
        let mut rows = Vec::new();

        for (row_idx, row) in self.rows().enumerate() {
            let mut cells = Vec::new();

            for (col_idx, _) in row.enumerate() {
                let pt: Point = (row_idx as i64, col_idx as i64).into();
                let mut neighbor_count = 0;

                if self.at(pt).is_some_and(|t| matches!(t, Tile::Nothing)) {
                    cells.push(false);
                } else {
                    // count neighbors by checking in all 8 possible directions
                    for dir in Dir8::all() {
                        let neighbor = pt + dir.step();
                        let at = self.at(neighbor);
                        if at.is_some_and(|t| matches!(t, Tile::Roll)) {
                            neighbor_count += 1;
                        }
                    }
                    cells.push(neighbor_count < 4);
                }
            }

            rows.push(cells);
        }

        Grid(rows)
    }
}
