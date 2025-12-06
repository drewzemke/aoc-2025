use itertools::Itertools;

pub mod puzzle06a;
pub mod puzzle06b;

pub enum Op {
    Add,
    Mult,
}

pub struct Column {
    nums: Vec<i64>,
    op: Op,
}

impl Column {
    pub fn value(&self) -> i64 {
        match self.op {
            Op::Add => self.nums.iter().sum(),
            Op::Mult => self.nums.iter().product(),
        }
    }
}

pub struct Worksheet(Vec<Column>);

impl Worksheet {
    pub fn parse(input: &str) -> Self {
        let mut cols = Vec::<Vec<i64>>::new();

        // all but the last line are nums, the last line is ops
        let num_lines = input.lines().dropping_back(1);
        for line in num_lines {
            let nums = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
            for (idx, num) in nums.enumerate() {
                if let Some(col) = cols.get_mut(idx) {
                    col.push(num);
                } else {
                    cols.push(Vec::from([num]));
                }
            }
        }

        let op_line = input.lines().last().unwrap();
        let cols = cols
            .into_iter()
            .zip(op_line.split_whitespace())
            .map(|(nums, op)| {
                let op = match op {
                    "+" => Op::Add,
                    "*" => Op::Mult,
                    _ => unreachable!(),
                };
                Column { nums, op }
            })
            .collect();

        Self(cols)
    }
}
