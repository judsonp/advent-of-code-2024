use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for found in re.captures_iter(input) {
        let (_, [left, right]) = found.extract();
        let left = left.parse::<u64>().unwrap();
        let right = right.parse::<u64>().unwrap();
        sum += left * right;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    for found in re.captures_iter(input) {
        let needle = found.get(0).unwrap().as_str();
        if needle.starts_with("don't") {
            enabled = false;
        } else if needle.starts_with("do") {
            enabled = true;
        } else if enabled {
            let left = found.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let right = found.get(2).unwrap().as_str().parse::<u64>().unwrap();
            sum += left * right;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part_one(&example);
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let example = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(&example);
        assert_eq!(result, Some(48));
    }
}
