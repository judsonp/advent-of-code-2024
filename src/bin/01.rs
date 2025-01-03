use std::sync::OnceLock;

use itertools::Itertools;

advent_of_code::solution!(1);

const INPUT_SIZE: usize = 1000;

// Use this OnceLock to cache the parsed and sorted input.
#[allow(dead_code)]
static COLUMNS: OnceLock<(Vec<u32>, Vec<u32>)> = OnceLock::new();

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::with_capacity(INPUT_SIZE);
    let mut right = Vec::with_capacity(INPUT_SIZE);
    // Each line is formatted as "<number> <number>".
    // Split these into two vectors of numbers, one for each column.
    input
        .lines()
        .map(|s| s.split_ascii_whitespace().next_tuple().unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .for_each(|(a, b)| {
            left.push(a);
            right.push(b);
        });
    // Sort the columns.
    left.sort();
    right.sort();
    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    // let (left, right) = &COLUMNS.get_or_init(|| parse_input(input));
    let (left, right) = parse_input(input);

    // The solution is the sum of pairwise absolute differnces in the sorted lists.
    let result = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // let (left, right) = &COLUMNS.get_or_init(|| parse_input(input));
    let (left, right) = parse_input(input);

    let mut sum: u64 = 0;
    let mut lidx = 0;
    let mut ridx = 0;

    // Run through the sorted lists, finding when a number is in both lists.
    while lidx < left.len() && ridx < right.len() {
        match left[lidx].cmp(&right[ridx]) {
            std::cmp::Ordering::Less => lidx += 1,
            std::cmp::Ordering::Greater => ridx += 1,
            std::cmp::Ordering::Equal => {
                // Count how many times the number appears in each list.
                let mut lcount = 0;
                while lidx + lcount < left.len() && left[lidx + lcount] == right[ridx] {
                    lcount += 1;
                }

                let mut rcount = 0;
                while ridx + rcount < right.len() && right[ridx + rcount] == left[lidx] {
                    rcount += 1;
                }

                // The contribution to the total is value * count_left * count_right.
                sum += (left[lidx] as u64) * (lcount as u64) * (rcount as u64);

                // Advance both lists.
                lidx += lcount;
                ridx += rcount;
            }
        }
    }

    Some(sum)

    // A more natural approach would be to use a counter on the right-hand column
    // for how often we've seen each number, but just sorting the two columns is
    // faster. We can use the approach above to get counts from sorted lists.
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
