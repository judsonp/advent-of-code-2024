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

    // Note: it's possible to do this in a single pass.
    // The nominal result is the sum of each element in the left-hand column mulitplied
    // by the number of times it appears in the right-hand column.
    // This is symmetric, so the result is the sum of value * left_count * right_count.
    // If you keep track of the number of elements on both sides, you can do this iteratively.
    // When you add a value to the left-hand column, increase the sum by
    //     (value * (left_count + 1) * right_count) - (value * left_count * right_count)
    //   which is just
    //      value * right_count
    // Similarly, when you add a value to the right-hand column, increase the sum by
    //    (value * left_count * (right_count + 1)) - (value * left_count * right_count)
    //  which is just
    //    value * left_count
    // A naive implementation of this turned out to be slower than the two-pass version.
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
