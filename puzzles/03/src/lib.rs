pub mod puzzle03a;
pub mod puzzle03b;

pub struct BatteryBank(Vec<u64>);

impl BatteryBank {
    pub fn parse(input: &str) -> Self {
        let vals = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        Self(vals)
    }

    // find the largest `length`-digit number that can be formed by digits in the battery (in order)
    pub fn max_joltage(&self, length: usize) -> u64 {
        let els = &self.0;

        let mut digits = Vec::new();

        let mut search_start = 0;

        for buffer in (1..=length).rev() {
            let search_end = els.len() - buffer;
            let max = els[search_start..=search_end].iter().max().unwrap();

            digits.push(max);

            let max_pos = search_start
                + els[search_start..=search_end]
                    .iter()
                    .position(|v| v == max)
                    .unwrap();

            search_start = max_pos + 1;
        }

        // make a number
        digits
            .into_iter()
            .enumerate()
            .map(|(idx, d)| d * 10u64.pow((length - idx - 1) as u32))
            .sum()
    }
}
