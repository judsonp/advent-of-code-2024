use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let start_values = parse_input(input);
    let end_values = start_values
        .into_iter()
        .map(|value| secret_value(value, 2000));
    let result = end_values.sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let start_values = parse_input(input);

    // secret_values[i] is the list of 2000+1 secret values for vendor i
    let secret_values = start_values
        .into_iter()
        .map(|start| secret_values(start, 2000))
        .collect_vec();

    // prices[i] is the list of 2000+1 prices for vendor i
    let prices = secret_values
        .into_iter()
        .map(|secrets| {
            secrets
                .into_iter()
                .map(|secret| (secret % 10) as i8)
                .collect_vec()
        })
        .collect_vec();

    // deltas[i] is a list for each vendor where
    // deltas[i][j] is the price difference from prices[i][j] to prices[i][j+1] (so, 2000)
    let deltas = prices
        .iter()
        .map(|pricelist| {
            pricelist
                .iter()
                .cloned()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec()
        })
        .collect_vec();

    // price_map[i] is, for vendor i,
    // a map of (four-delta sequence) to (applicable price)
    // where the appicable price is the price associated with the
    // first tie that sequence appears (for that vendor)
    let price_map = prices
        .iter()
        .zip(deltas.iter())
        .map(|(price_list, delta_list)| vendor_price_map(price_list, delta_list))
        .collect_vec();

    let possible_sequences = price_map
        .iter()
        .flat_map(|map| map.keys())
        .collect::<HashSet<_>>();

    let sequence_values = possible_sequences
        .into_iter()
        .map(|sequence| {
            (
                sequence,
                price_map
                    .iter()
                    .map(|map| map.get(sequence).cloned().unwrap_or(0) as u64)
                    .sum::<u64>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let best_value = *sequence_values.values().max().unwrap();

    Some(best_value)
}

type DeltaSequence = [i8; 4];
type VendorPriceMap = HashMap<DeltaSequence, i8>;

fn vendor_price_map(price_list: &[i8], delta_list: &[i8]) -> VendorPriceMap {
    let mut map = VendorPriceMap::new();

    assert!(price_list.len() == delta_list.len() + 1);

    for i in 3..delta_list.len() {
        let sequence: &[i8] = &delta_list[i - 3..=i];
        if !map.contains_key(sequence) {
            map.insert(sequence.try_into().unwrap(), price_list[i + 1]);
        }
    }

    map
}

fn secret_value(initial_value: u64, iterations: u32) -> u64 {
    let mut value = initial_value;
    for _ in 0..iterations {
        value = iterate(value);
    }
    value
}

fn secret_values(initial_value: u64, iterations: u32) -> Vec<u64> {
    let mut values = Vec::new();
    let mut value = initial_value;
    values.push(value);
    for _ in 0..iterations {
        value = iterate(value);
        values.push(value);
    }
    values
}

fn iterate(mut value: u64) -> u64 {
    value = prune(mix(value, value * 64));
    value = prune(mix(value, value / 32));
    value = prune(mix(value, value * 2048));
    value
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(v: u64) -> u64 {
    v % 16777216
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
