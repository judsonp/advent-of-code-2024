use counter::Counter;
use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    input
        .lines()
        .map(|s| s.split_ascii_whitespace().next_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .for_each(|(a, b)| {
            left.push(a);
            right.push(b);
        });
    left.sort();
    right.sort();
    let result = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left = Vec::with_capacity(1000);
    let mut right: Counter<i64, u64> = Counter::with_capacity(1000);
    input
        .lines()
        .map(|s| s.split_ascii_whitespace().next_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .for_each(|(a, b)| {
            left.push(a);
            right[&b] += 1;
        });
    let result = left.iter().map(|v| (*v as u64) * right[v]).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
