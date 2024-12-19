use std::collections::HashMap;

use advent_of_code::util::iter::CountIfParallel as _;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let result = input
        .patterns
        .par_iter()
        .count_if(|pattern| can_make_pattern(pattern, &input.towels, &mut HashMap::new()));

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let result = input
        .patterns
        .par_iter()
        .map(|pattern| ways_to_make_pattern(pattern, &input.towels, &mut HashMap::new()))
        .sum();

    Some(result)
}

fn can_make_pattern<'a>(
    pattern: &'a str,
    towels: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    if let Some(result) = memo.get(pattern) {
        return *result;
    }

    let result = towels
        .iter()
        .filter(|towel| pattern.starts_with(**towel))
        .any(|towel| can_make_pattern(&pattern[towel.len()..], towels, memo));

    memo.insert(pattern, result);
    result
}

fn ways_to_make_pattern<'a>(
    pattern: &'a str,
    towels: &[&str],
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if pattern.len() == 0 {
        return 1;
    }

    if let Some(result) = memo.get(pattern) {
        return *result;
    }

    let result = towels
        .iter()
        .filter(|towel| pattern.starts_with(**towel))
        .map(|towel| ways_to_make_pattern(&pattern[towel.len()..], towels, memo))
        .sum();

    memo.insert(pattern, result);
    result
}

struct Input<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let towels = lines.next().unwrap().split(", ").collect();
    lines.next().unwrap(); // newline
    let patterns = lines.collect();

    Input { towels, patterns }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
