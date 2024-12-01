use std::ops::Neg;

use num::{one, zero, One, Zero};

use super::point::Point2D;

pub enum Direction {
    East,
    North,
    West,
    South,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::North,
    Direction::West,
    Direction::South,
];

impl<T> From<Direction> for Point2D<T>
where
    T: One + Zero + Neg<Output = T>,
{
    fn from(value: Direction) -> Self {
        match value {
            Direction::East => Point2D::new(one(), zero()),
            Direction::North => Point2D::new(zero(), one()),
            Direction::West => Point2D::new(-one::<T>(), zero()),
            Direction::South => Point2D::new(zero(), -one::<T>()),
        }
    }
}