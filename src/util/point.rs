use itertools::Itertools;
use num::{Signed, Zero};
use std::{
    fmt::{self, Display},
    iter::Sum,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point<T, const D: usize> {
    pub values: [T; D],
}

pub type Point2D<T> = Point<T, 2>;
pub type Point3D<T> = Point<T, 3>;

impl<T, const D: usize> Point<T, D> {
    pub fn x(&self) -> T
    where
        T: Copy,
    {
        self.values[0]
    }

    pub fn y(&self) -> T
    where
        T: Copy,
    {
        self.values[1]
    }

    pub fn z(&self) -> T
    where
        T: Copy,
    {
        self.values[2]
    }
}

impl<T> Point<T, 2> {
    pub fn new(x: T, y: T) -> Self {
        Point { values: [x, y] }
    }

    pub fn from((x, y): (T, T)) -> Self {
        Point { values: [x, y] }
    }
}

impl<T> Point<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point { values: [x, y, z] }
    }

    pub fn from((x, y, z): (T, T, T)) -> Self {
        Point { values: [x, y, z] }
    }
}

impl<T, const D: usize> Point<T, D> {
    pub fn zero() -> Self
    where
        T: Zero + Copy,
    {
        Point {
            values: [T::zero(); D],
        }
    }

    pub fn convert<U>(self) -> Point<U, D>
    where
        U: From<T>,
    {
        Point {
            values: self.values.map(|v| U::from(v)),
        }
    }
}

impl<T, const D: usize> Add for Point<T, D>
where
    T: Add,
{
    type Output = Point<T::Output, D>;

    fn add(self, other: Point<T, D>) -> Point<T::Output, D> {
        Point {
            values: array_init::from_iter(
                self.values
                    .into_iter()
                    .zip(other.values)
                    .map(|(a, b)| a + b),
            )
            .unwrap(),
        }
    }
}

impl<T, const D: usize> Sub for Point<T, D>
where
    T: Sub,
{
    type Output = Point<T::Output, D>;

    fn sub(self, other: Point<T, D>) -> Point<T::Output, D> {
        Point {
            values: array_init::from_iter(
                self.values
                    .into_iter()
                    .zip(other.values)
                    .map(|(a, b)| a - b),
            )
            .unwrap(),
        }
    }
}

impl<T, const D: usize> Point<T, D>
where
    T: Mul + Copy,
{
    pub fn scale(self, other: T) -> Point<T::Output, D> {
        Point {
            values: array_init::map_array_init(&self.values, |v| *v * other),
        }
    }
}

impl<T, const D: usize> Point<T, D> {
    pub fn distance(self, other: Point<T, D>) -> f64
    where
        T: Sub,
        T::Output: Into<f64>,
    {
        self.values
            .into_iter()
            .zip(other.values)
            .map(|(a, b)| {
                let dx: f64 = (a - b).into();
                dx * dx
            })
            .sum::<f64>()
            .sqrt()
    }

    pub fn manhattan_distance(self, other: Point<T, D>) -> T::Output
    where
        T: Sub,
        T::Output: Signed + Sum,
    {
        self.values
            .into_iter()
            .zip(other.values)
            .map(|(a, b)| (a - b).abs())
            .sum()
    }
}

impl<T, const D: usize> Display for Point<T, D>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.values.iter().map(|v| v.to_string()).join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_add() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(3, 4);
        let result = a + b;
        assert_eq!(result, Point2D::new(4, 6));
    }

    #[test]
    fn float_add() {
        let a = Point2D::new(1.0, 2.0);
        let b = Point2D::new(3.0, 4.0);
        let result = a + b;
        assert_eq!(result, Point2D::new(4.0, 6.0));
    }

    #[test]
    fn mixed_add() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(3.0, 4.0);
        let result = a.convert() + b;
        assert_eq!(result, Point2D::new(4.0, 6.0));
    }

    #[test]
    fn sub() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(3, 4);
        let result = a - b;
        assert_eq!(result, Point2D::new(-2, -2));
    }

    #[test]
    fn scale() {
        let a = Point2D::new(1, 2);
        let result = a.scale(3);
        assert_eq!(result, Point2D::new(3, 6));
    }

    #[test]
    fn distance() {
        let a = Point2D::new(1.0, 2.0);
        let b = Point2D::new(4.0, 6.0);
        let result = a.distance(b);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn manhattan_distance() {
        let a = Point2D::new(1, 2);
        let b = Point2D::new(4, 6);
        let result = a.manhattan_distance(b);
        assert_eq!(result, 7);
    }

    #[test]
    fn display() {
        let a = Point2D::new(1, 2);
        assert_eq!(a.to_string(), "(1, 2)");
    }

    #[test]
    fn math_3d() {
        let a = Point3D::new(1, 2, 3);
        let b = Point3D::new(4, 5, 6);
        let result = a + b;
        assert_eq!(result, Point3D::new(5, 7, 9));
        assert_eq!(result.scale(3), Point3D::new(15, 21, 27));
        assert_eq!(a.manhattan_distance(b), 9);
    }
}
