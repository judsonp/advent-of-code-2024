use std::collections::HashMap;

use bit_set::BitSet;

advent_of_code::solution!(5);

const MAX_PAGES: usize = 100;

struct Rule {
    before: u32,
    after: u32,
}

struct RuleMap(HashMap<u32, BitSet>);

type Update = Vec<u32>;

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let mut rules = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (before, after) = line.split_once('|').unwrap();
        rules.push(Rule {
            before: before.parse().unwrap(),
            after: after.parse().unwrap(),
        });
    }
    let updates = lines
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    Input { rules, updates }
}

fn update_is_valid(update: &Update, rule_map: &RuleMap) -> bool {
    let mut disallowed_values: BitSet = BitSet::with_capacity(MAX_PAGES);

    for &val in update.iter() {
        if disallowed_values.contains(val as usize) {
            return false;
        }
        if let Some(new_disallowed) = rule_map.get(&val) {
            disallowed_values.union_with(new_disallowed);
        }
    }

    true
}

fn reorder_pages(update: &Update, rules: &[Rule]) -> Update {
    let mut reordered = Vec::with_capacity(update.len());

    for &value in update {
        let applicable_rules = rules
            .iter()
            .filter(|rule| rule.before == value)
            .collect::<Vec<_>>();
        let index = applicable_rules
            .iter()
            .filter_map(|rule| reordered.iter().position(|&x| x == rule.after))
            .min();
        if let Some(index) = index {
            reordered.insert(index, value);
        } else {
            reordered.push(value);
        }
    }

    reordered
}

pub fn part_one(input: &str) -> Option<u64> {
    let Input { rules, updates } = parse_input(input);

    let rule_map = RuleMap::from_iter(rules.iter().map(|rule| (rule.after, rule.before)));

    let result = updates
        .iter()
        .filter(|update| update_is_valid(update, &rule_map))
        .map(|update| update[update.len() / 2] as u64)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let Input { rules, updates } = parse_input(input);

    let rule_map = RuleMap::from_iter(rules.iter().map(|rule| (rule.after, rule.before)));

    let result = updates
        .iter()
        .filter(|update| !update_is_valid(update, &rule_map))
        .map(|update| reorder_pages(update, &rules))
        .map(|update| update[update.len() / 2] as u64)
        .sum();

    Some(result)
}

impl RuleMap {
    fn from_iter(rules: impl Iterator<Item = (u32, u32)>) -> RuleMap {
        let mut rule_map = RuleMap(HashMap::new());
        for (after, before) in rules {
            rule_map.0.entry(after).or_default().insert(before as usize);
        }
        rule_map
    }

    fn get(&self, key: &u32) -> Option<&BitSet> {
        self.0.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let input = parse_input(input);
        assert_eq!(input.rules.len(), 21);
        assert_eq!(input.updates.len(), 6);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
