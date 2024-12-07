use num::PrimInt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn apply<T>(self, lhs: T, rhs: T) -> Option<T>
    where
        T: PrimInt,
    {
        match self {
            Operation::Add => lhs.checked_add(&rhs),
            Operation::Mul => lhs.checked_mul(&rhs),
            Operation::Concat => lhs.checked_mul(&digit_shift(rhs)?)?.checked_add(&rhs),
        }
    }
}

fn digit_shift<T>(mut n: T) -> Option<T>
where
    T: PrimInt,
{
    let mut d = T::one();
    while n >= T::from(10).unwrap() {
        d = d * T::from(10).unwrap();
        n = n / T::from(10).unwrap();
    }
    d.checked_mul(&T::from(10).unwrap())
}

#[derive(Debug, Clone)]
struct PermutationIterator<'a, T> {
    items: &'a [T],
    current: Vec<usize>,
    done: bool,
}

impl<'a, T> PermutationIterator<'a, T> {
    fn new(items: &'a [T], size: usize) -> PermutationIterator<'a, T> {
        let current = vec![0; size];
        PermutationIterator {
            items,
            current,
            done: false,
        }
    }
}

impl<T> Iterator for PermutationIterator<'_, T>
where
    T: Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current.iter().map(|&i| self.items[i]).collect();
        let mut i = 0;
        while i < self.current.len() {
            self.current[i] += 1;
            if self.current[i] == self.items.len() {
                self.current[i] = 0;
                i += 1;
            } else {
                break;
            }
        }

        if i == self.current.len() {
            self.done = true;
        }

        Some(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Equation {
    result: u64,
    values: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result, values) = line.split_once(":").unwrap();
            let result = result.trim().parse().unwrap();
            let values = values
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            Equation { result, values }
        })
        .collect()
}

fn could_be_true(equation: &Equation, ops: &[Operation]) -> bool {
    let operation_permutations = PermutationIterator::new(ops, equation.values.len() - 1);

    for operations in operation_permutations {
        let mut result = Some(equation.values[0]);
        for (op, value) in operations.iter().zip(equation.values.iter().skip(1)) {
            result = op.apply(result.unwrap(), *value);
            if result.is_none() || result.unwrap() > equation.result {
                break;
            }
        }

        if result == Some(equation.result) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let ops = vec![Operation::Add, Operation::Mul];

    Some(
        equations
            .iter()
            .filter_map(|equation| {
                if could_be_true(equation, &ops) {
                    Some(equation.result)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let ops = vec![Operation::Add, Operation::Mul, Operation::Concat];

    Some(
        equations
            .par_iter()
            .filter_map(|equation| {
                if could_be_true(equation, &ops) {
                    Some(equation.result)
                } else {
                    None
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_shift() {
        assert_eq!(digit_shift(1000), Some(10000));
        assert_eq!(digit_shift(999), Some(1000));
        assert_eq!(digit_shift(123), Some(1000));
        assert_eq!(digit_shift(100), Some(1000));
        assert_eq!(digit_shift(99), Some(100));
        assert_eq!(digit_shift(1), Some(10));
        assert_eq!(digit_shift(0), Some(10));
    }

    #[test]
    fn test_concat() {
        assert_eq!(Operation::Concat.apply(1, 2), Some(12));
        assert_eq!(Operation::Concat.apply(12, 3), Some(123));
        assert_eq!(Operation::Concat.apply(123, 4), Some(1234));
        assert_eq!(Operation::Concat.apply(1234, 5), Some(12345));
        assert_eq!(Operation::Concat.apply(1, 23), Some(123));
        assert_eq!(Operation::Concat.apply(1, 234), Some(1234));
        assert_eq!(Operation::Concat.apply(1, 2345), Some(12345));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
