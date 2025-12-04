use crate::point::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir4 {
    North,
    South,
    East,
    West,
}

impl Dir4 {
    pub fn all() -> Vec<Self> {
        vec![Self::North, Self::South, Self::East, Self::West]
    }

    pub fn step(&self) -> Point {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
        .into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir8 {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

impl Dir8 {
    pub fn all() -> Vec<Self> {
        vec![
            Self::North,
            Self::South,
            Self::East,
            Self::West,
            Self::NorthEast,
            Self::SouthEast,
            Self::NorthWest,
            Self::SouthWest,
        ]
    }

    pub fn step(&self) -> Point {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
            Dir8::NorthEast => (-1, 1),
            Dir8::SouthEast => (1, 1),
            Dir8::NorthWest => (-1, -1),
            Dir8::SouthWest => (1, -1),
        }
        .into()
    }
}
