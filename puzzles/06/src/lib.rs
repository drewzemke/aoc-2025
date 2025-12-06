use itertools::Itertools;

pub mod puzzle06a;
pub mod puzzle06b;

#[derive(Debug)]
pub enum Op {
    Add,
    Mult,
}

#[derive(Debug)]
pub struct Parsed<T> {
    val: T,
    offset: usize,
    width: usize,
}

impl<T> Parsed<T> {
    pub fn new(val: T, offset: usize, width: usize) -> Self {
        Self { val, offset, width }
    }
}

#[derive(Debug)]
pub struct Column {
    nums: Vec<Parsed<i64>>,
    op: Parsed<Op>,
}

impl Column {
    pub fn value(&self) -> i64 {
        match self.op.val {
            Op::Add => self.nums.iter().map(|n| n.val).sum(),
            Op::Mult => self.nums.iter().map(|n| n.val).product(),
        }
    }

    // numbers are read top-down within each column
    //
    // 123
    //  45 => 356, 24, 1
    //   6
    //
    // 328
    // 64  => 8, 248, 369
    // 98
    pub fn cephalopod_value(&self) -> i64 {
        // strat: this is probably slow, but...
        // convert each number into an array of digits
        // read out the columnar digits, convert back to numbers
        // apply op

        let min_offset = self.nums.iter().map(|num| num.offset).min().unwrap();
        let max_width = self.nums.iter().map(|num| num.width).max().unwrap();

        // NOTE: taking advantage of the fact that the puzzle input contains
        // no zeros, so I can use zero as the blank placeholder
        // (otherwise I'd have to use an Option<i64> I suppose)
        let digits: Vec<_> = self
            .nums
            .iter()
            .map(|num| {
                let mut digits = vec![0; num.offset - min_offset];
                digits.extend(num_to_digits(num.val));
                digits.extend(vec![0; max_width - digits.len()]);
                digits
            })
            .collect();

        // all rows now have the same width
        let num_cols = digits[0].len();
        let num_rows = digits.len();

        let column_vals = (0..num_cols).map(|col_idx| {
            // skip leading and trailing zeros
            let col_digits: Vec<_> = (0..num_rows)
                .map(|row_idx| digits[row_idx][col_idx])
                .skip_while(|n| *n == 0)
                .take_while(|n| *n != 0)
                .collect();
            let num_digits = col_digits.len();
            col_digits
                .into_iter()
                .enumerate()
                .map(move |(idx, n)| n * 10_i64.pow((num_digits - idx - 1) as u32))
                .sum::<i64>()
        });

        match self.op.val {
            Op::Add => column_vals.sum(),
            Op::Mult => column_vals.product(),
        }
    }
}

/// extracts digits into an array, most significant first
fn num_to_digits(num: i64) -> Vec<i64> {
    let mut num = num;
    let mut digits = Vec::new();
    while num > 0 {
        let base = num.ilog10();
        let digit = num / 10i64.pow(base);
        digits.push(digit);
        num -= digit * 10_i64.pow(base);
    }
    digits
}

#[derive(Debug)]
pub struct Worksheet(Vec<Column>);

impl Worksheet {
    pub fn parse(input: &str) -> Self {
        let mut num_cols: Vec<Vec<_>> = Vec::new();

        // all but the last line are nums, the last line is ops
        for line in input.lines().dropping_back(1) {
            // start position of the string currently being parsed
            let mut start_pos = None;

            // keep track of the next column that we'll add to
            let mut col_idx = 0;

            // we add an extra space at the end of the line so that we trigger
            // the string parsing at the end of each line
            for (offset, char) in line.chars().chain([' ']).enumerate() {
                if char.is_whitespace() {
                    // if we just stopped scanning through a number,
                    // parse current string and save it to the column
                    if let Some(start) = start_pos
                        && offset > start
                    {
                        #[allow(clippy::char_indices_as_byte_indices)]
                        let num = line[start..offset].parse::<i64>().unwrap();
                        let num = Parsed::new(num, start, offset - start);

                        if let Some(col) = num_cols.get_mut(col_idx) {
                            col.push(num);
                        } else {
                            num_cols.push(Vec::from([num]));
                        }

                        col_idx += 1;
                        start_pos = None;
                    }
                } else {
                    // mark the start pos if we just started scanning a number
                    if start_pos.is_none() {
                        start_pos = Some(offset);
                    }
                }
            }
        }

        let op_line = input.lines().last().unwrap();
        let mut num_cols = num_cols.into_iter();

        let mut cols = Vec::new();

        for (offset, char) in op_line.chars().enumerate() {
            if !char.is_whitespace() {
                // complete the column with the op
                let op = match char {
                    '+' => Op::Add,
                    '*' => Op::Mult,
                    _ => unreachable!(),
                };
                let op = Parsed::new(op, offset, 1);
                let nums = num_cols.next().unwrap();
                let col = Column { nums, op };
                cols.push(col);
            }
        }

        Self(cols)
    }
}
