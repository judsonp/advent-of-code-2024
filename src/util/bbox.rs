use std::{
    fmt::{self, Display, Formatter},
    ops::{Mul, Sub},
};

use num::{one, Num};

use super::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BoundingBox<T, const D: usize>
where
    T: PartialOrd,
{
    lower: Point<T, D>,
    upper: Point<T, D>,
}

pub type BoundingBox2D<T> = BoundingBox<T, 2>;
pub type BoundingBox3D<T> = BoundingBox<T, 3>;

impl<T, const D: usize> BoundingBox<T, D>
where
    T: PartialOrd,
{
    pub fn new(mut lower: Point<T, D>, mut upper: Point<T, D>) -> Self {
        for i in 0..D {
            if lower.values[i] > upper.values[i] {
                std::mem::swap(&mut lower.values[i], &mut upper.values[i]);
            }
        }
        BoundingBox { lower, upper }
    }

    pub fn from((lower, upper): (Point<T, D>, Point<T, D>)) -> Self {
        BoundingBox::new(lower, upper)
    }

    pub fn lower(&self) -> &Point<T, D> {
        &self.lower
    }

    pub fn upper(&self) -> &Point<T, D> {
        &self.upper
    }

    pub fn contains(&self, point: &Point<T, D>) -> bool {
        for i in 0..D {
            if point.values[i] < self.lower.values[i] || point.values[i] > self.upper.values[i] {
                return false;
            }
        }
        true
    }

    pub fn intersects(&self, other: &BoundingBox<T, D>) -> bool {
        for i in 0..D {
            if self.upper.values[i] < other.lower.values[i]
                || self.lower.values[i] > other.upper.values[i]
            {
                return false;
            }
        }
        true
    }
}

impl<T, const D: usize> BoundingBox<T, D>
where
    T: PartialOrd + Copy,
{
    pub fn intersection(&self, other: &BoundingBox<T, D>) -> Option<BoundingBox<T, D>> {
        let lower = Point {
            values: array_init::from_iter(
                self.lower
                    .values
                    .iter()
                    .zip(other.lower.values.iter())
                    .map(|(&a, &b)| if a > b { a } else { b }),
            )
            .unwrap(),
        };
        let upper = Point {
            values: array_init::from_iter(
                self.upper
                    .values
                    .iter()
                    .zip(other.upper.values.iter())
                    .map(|(&a, &b)| if a < b { a } else { b }),
            )
            .unwrap(),
        };

        if lower
            .values
            .iter()
            .zip(upper.values.iter())
            .any(|(l, u)| l >= u)
        {
            None
        } else {
            Some(BoundingBox::new(lower, upper))
        }
    }
}

impl<T, const D: usize> BoundingBox<T, D>
where
    T: Num + PartialOrd + Copy,
{
    pub fn area(&self) -> <<T as Sub>::Output as Mul>::Output {
        let mut area = one();
        for i in 0..D {
            area = area * (self.upper.values[i] - self.lower.values[i]);
        }
        area
    }
}

impl Display for BoundingBox2D<i64> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.lower().x(),
            self.lower().y(),
            self.upper().x(),
            self.upper().y()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::util::point::Point2D;

    use super::*;

    #[test]
    fn test_bounding_box_new() {
        let lower = Point2D::new(0, 0);
        let upper = Point2D::new(1, 1);
        let bbox = BoundingBox2D::new(lower, upper);
        assert_eq!(bbox.lower(), &Point2D::new(0, 0));
        assert_eq!(bbox.upper(), &Point2D::new(1, 1));
    }

    #[test]
    fn test_bounding_box_contains() {
        let lower = Point2D::new(0.0, 0.0);
        let upper = Point2D::new(1.0, 1.0);
        let bbox = BoundingBox2D::new(lower, upper);
        assert!(bbox.contains(&Point2D::new(0, 0).convert()));
        assert!(bbox.contains(&Point2D::new(1, 1).convert()));
        assert!(bbox.contains(&Point2D::new(0.5, 0.5)));
        assert!(!bbox.contains(&Point2D::new(-1, 0).convert()));
        assert!(!bbox.contains(&Point2D::new(0, 2).convert()));
    }

    #[test]
    fn test_bounding_box_intersects() {
        let bbox1 = BoundingBox2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
        let bbox2 = BoundingBox2D::new(Point2D::new(0.5, 0.5), Point2D::new(1.5, 1.5));
        let bbox3 = BoundingBox2D::new(Point2D::new(2.0, 2.0), Point2D::new(3.0, 3.0));
        assert!(bbox1.intersects(&bbox2));
        assert!(!bbox1.intersects(&bbox3));
    }

    #[test]
    fn test_bounding_box_intersection() {
        let bbox1 = BoundingBox2D::new(Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0));
        let bbox2 = BoundingBox2D::new(Point2D::new(0.5, 0.5), Point2D::new(1.5, 1.5));
        let bbox3 = BoundingBox2D::new(Point2D::new(2.0, 2.0), Point2D::new(3.0, 3.0));
        assert_eq!(
            bbox1.intersection(&bbox2),
            Some(BoundingBox2D::new(
                Point2D::new(0.5, 0.5),
                Point2D::new(1.0, 1.0)
            ))
        );
        assert_eq!(bbox1.intersection(&bbox3), None);
    }

    #[test]
    fn test_bounding_box_area() {
        let bbox = BoundingBox2D::new(Point2D::new(0, 0), Point2D::new(2, 3));
        assert_eq!(bbox.area(), 6);
    }
}
