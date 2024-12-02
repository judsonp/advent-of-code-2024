use std::cmp;

use itertools::Itertools;

advent_of_code::solution!(2);

fn report_is_safe(report: impl Iterator<Item = u32>) -> bool {
    let mut direction = None;

    for (a, b) in report.tuple_windows() {
        let cmp = a.cmp(&b);
        match (cmp, direction) {
            (cmp::Ordering::Less, Some(cmp::Ordering::Greater)) => return false,
            (cmp::Ordering::Greater, Some(cmp::Ordering::Less)) => return false,
            (cmp::Ordering::Less, None) => direction = Some(cmp::Ordering::Less),
            (cmp::Ordering::Greater, None) => direction = Some(cmp::Ordering::Greater),
            _ => (),
        }
        if a.abs_diff(b) > 3 || a.abs_diff(b) < 1 {
            return false;
        }
    }

    true
}

fn report_safe(report: &[u32]) -> bool {
    report_is_safe(report.iter().copied())
}

fn report_safe_omitting_one(report: &[u32]) -> bool {
    if report_is_safe(report.iter().copied()) {
        return true;
    }

    for i in 0..report.len() {
        if report_is_safe(report.iter().copied().enumerate().filter_map(|(j, v)| {
            if i == j {
                None
            } else {
                Some(v)
            }
        })) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    Some(reports.iter().filter(|r| report_safe(r)).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    Some(
        reports
            .iter()
            .filter(|r| report_safe_omitting_one(r))
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
