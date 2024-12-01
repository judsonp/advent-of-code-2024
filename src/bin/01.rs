use counter::Counter;
use itertools::Itertools;

advent_of_code::solution!(1);

const INPUT_SIZE: usize = 1000;

pub fn part_one(input: &str) -> Option<u64> {
    let mut left = Vec::with_capacity(INPUT_SIZE);
    let mut right = Vec::with_capacity(INPUT_SIZE);
    // Each line is formatted as "<number> <number>".
    // Split these into two vectors of numbers, one for each column.
    input
        .lines()
        .map(|s| s.split_ascii_whitespace().next_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .for_each(|(a, b)| {
            left.push(a);
            right.push(b);
        });
    // Sort the columns.
    left.sort();
    right.sort();
    // The solution is the sum of pairwise absolute differnces in the sorted lists.
    let result = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left = Vec::with_capacity(INPUT_SIZE);
    let mut right: Counter<i64, u64> = Counter::with_capacity(INPUT_SIZE);
    // Parse the two columns, as in Part 1.
    // For the right-hand column, track the number of times each value appears.
    input
        .lines()
        .map(|s| s.split_ascii_whitespace().next_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .for_each(|(a, b)| {
            left.push(a);
            right[&b] += 1;
        });
    // The solution is the sum of each element in the left-hand column
    // multiplied by the number of times it appears in the right-hand column.
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
