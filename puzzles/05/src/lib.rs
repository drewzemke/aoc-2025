use std::ops::RangeInclusive;

pub mod puzzle05a;
pub mod puzzle05b;

#[derive(Debug)]
pub struct IngredientDb {
    pub ranges: Vec<RangeInclusive<u64>>,
    pub ingredients: Vec<u64>,
}

impl IngredientDb {
    pub fn parse(input: &str) -> Self {
        let (ranges, ingredients) = input.split_once("\n\n").unwrap();

        let ranges = ranges
            .trim()
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            })
            .collect();

        let ingredients = ingredients
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        Self {
            ranges,
            ingredients,
        }
    }

    pub fn merge_ranges(&mut self) {
        // sort ranges by first entry, then
        // proceed through in order and build connect ranges
        self.ranges
            .sort_unstable_by(|r1, r2| r1.start().cmp(r2.start()));

        let mut new_ranges = Vec::new();
        let mut current: Option<RangeInclusive<u64>> = None;

        for range in &self.ranges {
            if let Some(ref other) = current {
                // can we merge `range` into `other`? if so, do it mannn
                if range.start() <= other.end() {
                    let start = *other.start();
                    let end = *range.end().max(other.end());
                    current = Some(start..=end);
                }
                // otherwise, put the current range in the bag and put start a new one
                else {
                    new_ranges.push(other.clone());
                    current = Some(range.clone());
                }
            } else {
                // if there's no current range, put this one, done
                current = Some(range.clone());
            }
        }

        // if there's still a working range, add it to the list
        if let Some(current) = current {
            new_ranges.push(current);
        }

        self.ranges = new_ranges;
    }
}
