use std::collections::HashMap;

pub mod puzzle08a;
pub mod puzzle08b;

#[derive(Debug, Clone)]
pub struct JunctionBox(u64, u64, u64);

impl JunctionBox {
    pub fn parse(input: &str) -> Self {
        let mut nums = input.split(',');
        Self(
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
            nums.next().unwrap().parse().unwrap(),
        )
    }

    fn sq_dist(&self, other: &Self) -> u64 {
        self.0.abs_diff(other.0).pow(2)
            + self.1.abs_diff(other.1).pow(2)
            + self.2.abs_diff(other.2).pow(2)
    }

    pub fn make_circuits(boxes: &[Self], conns: usize) -> Vec<Vec<usize>> {
        // compute pair-wise distances
        let mut distances: Vec<(usize, usize, u64)> = boxes
            .iter()
            .enumerate()
            .flat_map(|(idx1, pt1)| {
                boxes
                    .iter()
                    .enumerate()
                    .skip(idx1 + 1)
                    .map(|(idx2, pt2)| (idx1, idx2, pt1.sq_dist(pt2)))
                    .collect::<Vec<_>>()
            })
            .collect();

        // keep track of connected components by mapping the index of each box to its component.
        let mut comp_map = HashMap::new();

        distances.sort_unstable_by_key(|(_, _, d)| *d);

        // find the two closest boxes that aren't already connected, make an edge between them
        let mut conns_made = 0;
        let mut num_comps = 0;
        for (idx1, idx2, _) in distances {
            // update the component map so that idx2 and everything in the same
            // component as idx2 are now in the component of idx1
            let c1 = *comp_map.entry(idx1).or_insert_with(|| {
                num_comps += 1;
                num_comps
            });

            // dbg!(c1);

            if let Some(c2) = comp_map.get(&idx2).cloned() {
                for (_, v) in comp_map.iter_mut() {
                    if *v == c2 {
                        *v = c1
                    }
                }
            } else {
                comp_map.insert(idx2, c1);
            }

            conns_made += 1;
            if conns_made >= conns {
                break;
            }
        }

        // compute the connected components of the graph
        let mut circuits = vec![Vec::new(); num_comps];

        for (idx, comp) in comp_map {
            // subtracting because we started counting components at 1
            circuits[comp - 1].push(idx);
        }

        circuits
    }
}
