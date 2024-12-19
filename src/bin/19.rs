use std::collections::HashMap;

use advent_of_code::util::iter::CountIfParallel as _;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let presorted_towels = presort_towels(&input.towels);
    let result = input
        .patterns
        .par_iter()
        .count_if(|pattern| can_make_pattern(pattern, &presorted_towels, &mut HashMap::new()));

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    let presorted_towels = presort_towels(&input.towels);
    let result = input
        .patterns
        .par_iter()
        .map(|pattern| ways_to_make_pattern(pattern, &presorted_towels, &mut HashMap::new()))
        .sum();

    Some(result)
}

fn presort_towels<'a>(towels: &[&'a str]) -> HashMap<char, Vec<&'a str>> {
    towels
        .iter()
        .map(|towel| (towel.chars().next().unwrap(), towel))
        .into_grouping_map()
        .collect()
}

fn can_make_pattern<'a>(
    pattern: &'a str,
    towels: &HashMap<char, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }

    if let Some(result) = memo.get(pattern) {
        return *result;
    }

    let relevant_towels = towels.get(&pattern.chars().next().unwrap());

    let result = if let Some(applicable_towels) = relevant_towels {
        applicable_towels
            .iter()
            .filter(|towel| pattern.starts_with(**towel))
            .any(|towel| can_make_pattern(&pattern[towel.len()..], towels, memo))
    } else {
        false
    };

    memo.insert(pattern, result);
    result
}

fn ways_to_make_pattern<'a>(
    pattern: &'a str,
    towels: &HashMap<char, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(result) = memo.get(pattern) {
        return *result;
    }

    let relevant_towels = towels.get(&pattern.chars().next().unwrap());

    let result = if let Some(applicable_towels) = relevant_towels {
        applicable_towels
            .iter()
            .filter(|towel| pattern.starts_with(**towel))
            .map(|towel| ways_to_make_pattern(&pattern[towel.len()..], towels, memo))
            .sum()
    } else {
        0
    };

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
