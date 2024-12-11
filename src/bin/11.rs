use memoize::memoize;
use num::PrimInt;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let result = input.iter().map(|&v| stones(v, 25)).sum();
    memoized_flush_stones();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let result = input.iter().map(|&v| stones(v, 75)).sum();
    memoized_flush_stones();
    Some(result)
}

#[memoize]
fn stones(value: u64, blinks: u8) -> u64 {
    if blinks == 0 {
        1
    } else if value == 0 {
        stones(1, blinks - 1)
    } else if let Some((left, right)) = split_digits(value) {
        stones(left, blinks - 1) + stones(right, blinks - 1)
    } else {
        stones(value * 2024, blinks - 1)
    }
}

fn digits(mut n: u64) -> u64 {
    let mut d = 1;
    while n >= 10 {
        d += 1;
        n /= 10;
    }
    d
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    let digits = digits(n) as u32;
    if digits % 2 != 0 {
        return None;
    }
    let shift = 10.pow(digits / 2);
    let left = n / shift;
    let right = n % shift;
    Some((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
