pub mod puzzle03a;
pub mod puzzle03b;

pub struct BatteryBank(Vec<u32>);

impl BatteryBank {
    pub fn parse(input: &str) -> Self {
        let vals = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
        Self(vals)
    }

    // find the largets two digit number that can be formed by digits in the battery (in order)
    pub fn max_joltage(&self) -> u32 {
        let els = &self.0;
        let mut max = els.iter().max().unwrap();
        let mut max_pos = els.iter().position(|v| v == max).unwrap();

        // if the max is at the end, we need to find the second largest
        if max_pos == els.len() - 1 {
            max = els[..els.len() - 1].iter().max().unwrap();
            max_pos = els.iter().position(|v| v == max).unwrap();
        }

        // find the next largest after the max
        let second_max = els[max_pos + 1..].iter().max().unwrap();

        // make a two-digit number
        max * 10 + *second_max
    }
}
