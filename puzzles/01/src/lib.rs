pub mod puzzle01a;
pub mod puzzle01b;

pub enum Dir {
    Right,
    Left,
}

pub struct DialRotation {
    dir: Dir,
    amount: i64,
}

impl DialRotation {
    pub fn parse(input: &str) -> Self {
        let char = input.chars().next().unwrap();
        let dir = match char {
            'R' => Dir::Right,
            'L' => Dir::Left,
            _ => unreachable!(),
        };

        let amount = input[1..].parse().unwrap();

        Self { dir, amount }
    }
}
