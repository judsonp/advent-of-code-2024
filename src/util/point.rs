use num::{Signed, Zero};
use std::{
    fmt::{self, Display},
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2D { x, y }
    }

    pub fn zero() -> Self
    where
        T: Zero,
    {
        Point2D {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn convert<U>(self) -> Point2D<U>
    where
        U: From<T>,
    {
        Point2D {
            x: U::from(self.x),
            y: U::from(self.y),
        }
    }
}

impl<T> From<(T, T)> for Point2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Point2D { x, y }
    }
}

impl<T> Add for Point2D<T>
where
    T: Add,
{
    type Output = Point2D<T::Output>;

    fn add(self, other: Point2D<T>) -> Point2D<T::Output> {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point2D<T>
where
    T: Sub,
{
    type Output = Point2D<T::Output>;

    fn sub(self, other: Point2D<T>) -> Point2D<T::Output> {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Point2D<T>
where
    T: Mul + Copy,
{
    pub fn scale(self, other: T) -> Point2D<T::Output> {
        Point2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> Point2D<T> {
    pub fn distance(self, other: Point2D<T>) -> f64
    where
        T: Sub,
        T::Output: Into<f64>,
    {
        let dx: f64 = (self.x - other.x).into();
        let dy: f64 = (self.y - other.y).into();
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance(self, other: Point2D<T>) -> T::Output
    where
        T: Sub,
        T::Output: Signed,
    {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T> Display for Point2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
}
