use common::{dir::Dir4, point::Point};
use itertools::Itertools;

pub mod puzzle09a;
pub mod puzzle09b;

#[derive(Debug)]
enum Corner {
    Q1,  // quandrant 1 filled in relative to this corner
    Q1C, // everything but quadrant 1 filled in
    Q2,
    Q2C,
    Q3,
    Q3C,
    Q4,
    Q4C,
}

impl Corner {
    pub fn contains_pt(&self, pt: &Point, from: &Point) -> bool {
        match self {
            // swapped y comparisons because that's how I treat things when
            // computing corners
            Corner::Q1 => pt.x() > from.x() && pt.y() < from.y(),
            Corner::Q1C => !(pt.x() > from.x() && pt.y() < from.y()),
            Corner::Q2 => pt.x() < from.x() && pt.y() < from.y(),
            Corner::Q2C => !(pt.x() < from.x() && pt.y() < from.y()),
            Corner::Q3 => pt.x() < from.x() && pt.y() > from.y(),
            Corner::Q3C => !(pt.x() < from.x() && pt.y() > from.y()),
            Corner::Q4 => pt.x() > from.x() && pt.y() > from.y(),
            Corner::Q4C => !(pt.x() > from.x() && pt.y() > from.y()),
        }
    }
}

#[derive(Debug)]
pub struct PointList(Vec<(Point, Corner)>);

impl PointList {
    pub fn parse(input: &str) -> Self {
        let points: Vec<Point> = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                let x = x.parse::<i64>().unwrap();
                let y = y.parse::<i64>().unwrap();
                (x, y).into()
            })
            .collect();

        let mut corners = Vec::new();

        // NOTE: this assumption may be wrong about the input
        let mut inside = Dir4::South;

        let extended_pts = points
            .iter()
            .skip(points.len() - 1)
            .chain(&points)
            .chain(points.iter().take(1))
            .collect_vec();

        for pts in extended_pts.windows(3) {
            let pt0 = pts[0];
            let pt1 = pts[1];
            let pt2 = pts[2];

            // println!("{pt0:?}--{pt1:?}--{pt2:?}");
            // println!("inside is {inside:?}");

            let corner = if pt0.x() == pt1.x() {
                if pt0.y() > pt1.y() {
                    if pt1.x() < pt2.x() {
                        // ->
                        // ↑
                        if inside == Dir4::East {
                            inside = Dir4::South;
                            Corner::Q4
                        } else {
                            inside = Dir4::North;
                            Corner::Q4C
                        }
                    } else {
                        // <-
                        //  ↑
                        if inside == Dir4::East {
                            inside = Dir4::North;
                            Corner::Q3C
                        } else {
                            inside = Dir4::South;
                            Corner::Q3
                        }
                    }
                } else {
                    #[allow(clippy::collapsible_else_if)]
                    if pt1.x() < pt2.x() {
                        // ↓
                        // ->
                        if inside == Dir4::East {
                            inside = Dir4::North;
                            Corner::Q1
                        } else {
                            inside = Dir4::South;
                            Corner::Q1C
                        }
                    } else {
                        //  ↓
                        // <-
                        if inside == Dir4::East {
                            inside = Dir4::South;
                            Corner::Q2C
                        } else {
                            inside = Dir4::North;
                            Corner::Q2
                        }
                    }
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if pt0.x() < pt1.x() {
                    if pt1.y() > pt2.y() {
                        //  ↑
                        // ->
                        if inside == Dir4::North {
                            inside = Dir4::West;
                            Corner::Q2
                        } else {
                            inside = Dir4::East;
                            Corner::Q2C
                        }
                    } else {
                        // ->
                        //  ↓
                        if inside == Dir4::North {
                            inside = Dir4::East;
                            Corner::Q3C
                        } else {
                            inside = Dir4::West;
                            Corner::Q3
                        }
                    }
                } else {
                    if pt1.y() > pt2.y() {
                        // ↑
                        // <-
                        if inside == Dir4::North {
                            inside = Dir4::East;
                            Corner::Q1
                        } else {
                            inside = Dir4::West;
                            Corner::Q1C
                        }
                    } else {
                        // <-
                        // ↓
                        if inside == Dir4::North {
                            inside = Dir4::West;
                            Corner::Q4C
                        } else {
                            inside = Dir4::East;
                            Corner::Q4
                        }
                    }
                }
            };

            // println!("adding {corner:?}");

            corners.push(corner);
        }

        let list = points.into_iter().zip(corners).collect();
        Self(list)
    }

    pub fn max_area(&self) -> i64 {
        // just... calculate the area of every pair and take the max
        self.0
            .iter()
            .enumerate()
            .flat_map(|(idx, (pt1, _))| {
                self.0.iter().skip(idx + 1).map(|(pt2, _)| {
                    let dx = (pt1.x() - pt2.x()).abs() + 1;
                    let dy = (pt1.y() - pt2.y()).abs() + 1;
                    dx * dy
                })
            })
            .max()
            .unwrap()
    }

    pub fn max_area_restricted(&self) -> i64 {
        // same but only allow rectangles that don't contain any other points
        // in their interior, and whose corners match the corners as read
        // when we parsed the path
        self.0
            .iter()
            .enumerate()
            .flat_map(|(idx, (pt1, cr1))| {
                // println!("{pt1:?} with {cr1:?}");
                self.0.iter().skip(idx + 1).filter_map(|(pt2, cr2)| {
                    // print!("  to {pt2:?} with {cr2:?}: ");

                    let min_x = pt1.x().min(pt2.x());
                    let max_x = pt1.x().max(pt2.x());
                    let x_range = (min_x + 1)..max_x;

                    let min_y = pt1.y().min(pt2.y());
                    let max_y = pt1.y().max(pt2.y());
                    let y_range = (min_y + 1)..max_y;

                    // make sure the interior of this rectangle does not contain any corners
                    let contains_other_pt = self
                        .0
                        .iter()
                        .find(|(pt3, _)| x_range.contains(&pt3.x()) && y_range.contains(&pt3.y()));

                    // make sure the interior of this rectangle does not contain any edges
                    let mut contains_edge = false;
                    for pair in self.0.windows(2) {
                        let (e0, _) = pair[0];
                        let (e1, _) = pair[1];

                        let edge_min_x = e0.x().min(e1.x());
                        let edge_max_x = e0.x().max(e1.x());

                        let edge_min_y = e0.y().min(e1.y());
                        let edge_max_y = e0.y().max(e1.y());

                        if e0.x() == e1.x() {
                            // vertical edge, so check it if it passes vertically through the rectangle
                            if x_range.contains(&e0.x()) && edge_min_y < min_y && edge_max_y > max_y
                            {
                                contains_edge = true;
                                break;
                            }
                        } else {
                            // horizontal edge
                            if y_range.contains(&e0.y()) && edge_min_x < min_x && edge_max_x > max_x
                            {
                                contains_edge = true;
                                break;
                            }
                        }
                    }

                    let corner_compatibilty =
                        cr1.contains_pt(pt2, pt1) && cr2.contains_pt(pt1, pt2);

                    if !corner_compatibilty {
                        // println!("nope, corners don't work");
                        None
                    } else if let Some(_pt3) = contains_other_pt {
                        // println!("nope, contains {pt3:?}");
                        None
                    } else if contains_edge {
                        // println!("nope, contains edge");
                        None
                    } else {
                        let dim1 = (pt1.x() - pt2.x()).abs() + 1;
                        let dim2 = (pt1.y() - pt2.y()).abs() + 1;
                        // println!("cool, area is {}", dim1 * dim2);
                        Some(dim1 * dim2)
                    }
                })
            })
            .max()
            .unwrap()
    }
}
