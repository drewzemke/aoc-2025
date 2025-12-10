use common::point::Point;

pub mod puzzle09a;
pub mod puzzle09b;

pub struct PointList(Vec<Point>);

impl PointList {
    pub fn parse(input: &str) -> Self {
        let list = input
            .lines()
            .map(|line| {
                let (row, col) = line.split_once(',').unwrap();
                let row = row.parse::<i64>().unwrap();
                let col = col.parse::<i64>().unwrap();
                (row, col).into()
            })
            .collect();

        Self(list)
    }

    pub fn max_area(&self) -> i64 {
        // just... calculate the area of every pair and take the max, idk
        self.0
            .iter()
            .enumerate()
            .flat_map(|(idx, pt1)| {
                self.0.iter().skip(idx + 1).map(|pt2| {
                    let dim1 = (pt1.row - pt2.row).abs() + 1;
                    let dim2 = (pt1.col - pt2.col).abs() + 1;
                    dim1 * dim2
                })
            })
            .max()
            .unwrap()
    }
}
