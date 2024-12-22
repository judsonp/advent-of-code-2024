use std::collections::HashMap;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator as _};
use smallvec::SmallVec;

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

    // price_map[i] is, for vendor i,
    // a map of (four-delta sequence) to (applicable price)
    // where the appicable price is the price associated with the
    // first tie that sequence appears (for that vendor)
    let price_maps = start_values
        .into_iter()
        .par_bridge()
        .map(|start| build_vendor_price_map(start, 2000))
        .collect::<Vec<_>>();

    let sequence_values = price_maps
        .into_iter()
        .flat_map(|map| map.into_iter())
        .into_grouping_map()
        .sum();

    let best_value = sequence_values.into_values().max().unwrap();

    Some(best_value as u64)
}

type DeltaSequence = [i8; 4];
type VendorPriceMap = HashMap<DeltaSequence, u32>;

fn build_vendor_price_map(initial_secret: u64, iterations: u32) -> VendorPriceMap {
    let mut secret = initial_secret;
    let mut previous_price = (initial_secret % 10) as i8;
    let mut sequence: SmallVec<[i8; 4]> = SmallVec::new();
    let mut map = VendorPriceMap::new();

    for _ in 0..iterations {
        secret = iterate(secret);
        let price = (secret % 10) as i8;
        let delta = price - previous_price;

        if sequence.len() == 4 {
            sequence.rotate_left(1);
            sequence.pop();
            sequence.push(delta);

            if !map.contains_key(&sequence[0..4]) {
                map.insert(
                    array_init::from_iter(sequence.iter().cloned()).unwrap(),
                    price as u32,
                );
            }
        } else {
            sequence.push(delta);
        }

        previous_price = price;
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
