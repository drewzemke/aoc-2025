pub mod puzzle01a;
pub mod puzzle01b;

#[derive(Debug)]
pub enum Dir {
    Right,
    Left,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::Right => write!(f, "R"),
            Dir::Left => write!(f, "L"),
        }
    }
}

#[derive(Debug)]
pub struct DialRotation {
    dir: Dir,
    amount: i64,
}

impl std::fmt::Display for DialRotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.dir, self.amount)
    }
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
