use std::ops::Add;

use num::Num;

use super::{bbox::BoundingBox, point::Point};

impl<T, const D: usize> Point<T, D>
where
    T: PartialOrd + Copy,
{
    pub fn clamp(self, bounds: &BoundingBox<T, D>) -> Self {
        let bounds = bounds
            .lower()
            .values
            .iter()
            .zip(bounds.upper().values.iter());
        let clamped = self
            .values
            .into_iter()
            .zip(bounds)
            .map(|(val, (&lower, &upper))| num::clamp(val, lower, upper));
        Point {
            values: array_init::from_iter(clamped).unwrap(),
        }
    }
}

impl<T, const D: usize> Point<T, D>
where
    T: Add + PartialOrd + Copy,
    <T as Add>::Output: PartialOrd + Copy,
{
    pub fn clamped_add(
        self,
        other: &Point<T, D>,
        bounds: &BoundingBox<<T as Add>::Output, D>,
    ) -> Point<<T as Add>::Output, D> {
        (self + *other).clamp(bounds)
    }
}

impl<T, const D: usize> Point<T, D>
where
    T: Num + PartialOrd + Copy,
{
    pub fn wrap(self, bounds: &BoundingBox<T, D>) -> Self {
        let bounds = bounds
            .lower()
            .values
            .iter()
            .zip(bounds.upper().values.iter());
        let wrapped = self
            .values
            .into_iter()
            .zip(bounds)
            .map(|(val, (&lower, &upper))| {
                let range = upper - lower;
                let val = (val - lower) % range;
                if val < num::zero() {
                    val + range + lower
                } else {
                    val + lower
                }
            });
        Point {
            values: array_init::from_iter(wrapped).unwrap(),
        }
    }

    pub fn wrapped_add(self, other: &Point<T, D>, bounds: &BoundingBox<T, D>) -> Point<T, D> {
        (self + *other).wrap(bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        let point = Point {
            values: [5, 15, 25],
        };
        let bounds = BoundingBox::new(
            Point {
                values: [0, 10, 20],
            },
            Point {
                values: [10, 20, 30],
            },
        );
        let clamped = point.clamp(&bounds);
        assert_eq!(clamped.values, [5, 15, 25]);

        let point = Point {
            values: [-5, 25, 35],
        };
        let clamped = point.clamp(&bounds);
        assert_eq!(clamped.values, [0, 20, 30]);
    }

    #[test]
    fn test_clamp_float() {
        let point = Point {
            values: [5.0, 15.0, 25.0],
        };
        let bounds = BoundingBox::new(
            Point {
                values: [0.0, 10.0, 20.0],
            },
            Point {
                values: [10.0, 20.0, 30.0],
            },
        );
        let clamped = point.clamp(&bounds);
        assert_eq!(clamped.values, [5.0, 15.0, 25.0]);

        let point = Point {
            values: [-5.0, 25.0, 35.0],
        };
        let clamped = point.clamp(&bounds);
        assert_eq!(clamped.values, [0.0, 20.0, 30.0]);
    }

    #[test]
    fn test_wrap() {
        let point = Point {
            values: [15, 25, 35],
        };
        let bounds = BoundingBox::new(
            Point {
                values: [0, 10, 20],
            },
            Point {
                values: [10, 20, 30],
            },
        );
        let wrapped = point.wrap(&bounds);
        assert_eq!(wrapped.values, [5, 15, 25]);

        let point = Point {
            values: [-5, 5, 15],
        };
        let wrapped = point.wrap(&bounds);
        assert_eq!(wrapped.values, [5, 15, 25]);
    }

    #[test]
    fn test_wrap_float() {
        let point = Point {
            values: [15.0, 25.0, 35.0],
        };
        let bounds = BoundingBox::new(
            Point {
                values: [0.0, 10.0, 20.0],
            },
            Point {
                values: [10.0, 20.0, 30.0],
            },
        );
        let wrapped = point.wrap(&bounds);
        assert_eq!(wrapped.values, [5.0, 15.0, 25.0]);

        let point = Point {
            values: [-5.0, 5.0, 15.0],
        };
        let wrapped = point.wrap(&bounds);
        assert_eq!(wrapped.values, [5.0, 15.0, 25.0]);
    }

    #[test]
    fn test_clamped_add() {
        let point1 = Point {
            values: [5, 15, 25],
        };
        let point2 = Point { values: [3, 4, 5] };
        let bounds = BoundingBox::new(
            Point {
                values: [0, 10, 20],
            },
            Point {
                values: [10, 20, 30],
            },
        );
        let clamped_add = point1.clamped_add(&point2, &bounds);
        assert_eq!(clamped_add.values, [8, 19, 30]);

        let point1 = Point {
            values: [5, 15, 25],
        };
        let point2 = Point {
            values: [10, 10, 10],
        };
        let clamped_add = point1.clamped_add(&point2, &bounds);
        assert_eq!(clamped_add.values, [10, 20, 30]);
    }

    #[test]
    fn test_clamped_add_float() {
        let point1 = Point {
            values: [5.0, 15.0, 25.0],
        };
        let point2 = Point {
            values: [3.0, 4.0, 5.0],
        };
        let bounds = BoundingBox::new(
            Point {
                values: [0.0, 10.0, 20.0],
            },
            Point {
                values: [10.0, 20.0, 30.0],
            },
        );
        let clamped_add = point1.clamped_add(&point2, &bounds);
        assert_eq!(clamped_add.values, [8.0, 19.0, 30.0]);

        let point1 = Point {
            values: [5.0, 15.0, 25.0],
        };
        let point2 = Point {
            values: [10.0, 10.0, 10.0],
        };
        let clamped_add = point1.clamped_add(&point2, &bounds);
        assert_eq!(clamped_add.values, [10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_wrapped_add() {
        let point1 = Point {
            values: [5, 15, 25],
        };
        let point2 = Point {
            values: [10, 10, 10],
        };
        let bounds = BoundingBox::new(
            Point {
                values: [0, 10, 20],
            },
            Point {
                values: [10, 20, 30],
            },
        );
        let wrapped_add = point1.wrapped_add(&point2, &bounds);
        assert_eq!(wrapped_add.values, [5, 15, 25]);

        let point1 = Point {
            values: [5, 15, 25],
        };
        let point2 = Point { values: [6, 7, 8] };
        let wrapped_add = point1.wrapped_add(&point2, &bounds);
        assert_eq!(wrapped_add.values, [1, 12, 23]);
    }

    #[test]
    fn test_wrapped_add_float() {
        let point1 = Point {
            values: [5.0, 15.0, 25.0],
        };
        let point2 = Point {
            values: [10.0, 10.0, 10.0],
        };
        let bounds = BoundingBox::new(
            Point {
                values: [0.0, 10.0, 20.0],
            },
            Point {
                values: [10.0, 20.0, 30.0],
            },
        );
        let wrapped_add = point1.wrapped_add(&point2, &bounds);
        assert_eq!(wrapped_add.values, [5.0, 15.0, 25.0]);

        let point1 = Point {
            values: [5.0, 15.0, 25.0],
        };
        let point2 = Point {
            values: [6.0, 7.0, 8.0],
        };
        let wrapped_add = point1.wrapped_add(&point2, &bounds);
        assert_eq!(wrapped_add.values, [1.0, 12.0, 23.0]);
    }
}
