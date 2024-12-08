use core::panic;
use std::{
    cmp::max,
    collections::HashSet,
    iter::{empty, once, successors},
};

use advent_of_code::util::{bbox::BoundingBox2D, point::Point2D};
use either::Either::{Left, Right};
use halfbrown::HashMap;
use itertools::Itertools;
use num::Integer;

advent_of_code::solution!(8);

struct Input {
    antennae: HashMap<char, Vec<Point2D<i32>>>,
    bounds: BoundingBox2D<i32>,
}

fn parse_input(input: &str) -> Input {
    let mut antennae: HashMap<char, Vec<Point2D<i32>>> = HashMap::new();
    let mut max_x = -1;
    let mut max_y = -1;
    for (y, line) in input.lines().enumerate() {
        max_y = y as i32;
        for (x, entry) in line.chars().enumerate() {
            max_x = max(max_x, x as i32);
            if entry == '.' {
                continue;
            } else if entry.is_alphabetic() || entry.is_numeric() {
                if let Some(locs) = antennae.get_mut(&entry) {
                    locs.push(Point2D::new(x as i32, y as i32));
                } else {
                    antennae.insert(entry, vec![Point2D::new(x as i32, y as i32)]);
                }
            } else {
                panic!("Unexpected character: {}", entry);
            }
        }
    }

    Input {
        antennae,
        bounds: BoundingBox2D::new(Point2D::new(0, 0), Point2D::new(max_x, max_y)),
    }
}

fn find_antinodes(loc_a: Point2D<i32>, loc_b: Point2D<i32>) -> impl Iterator<Item = Point2D<i32>> {
    // a + delta = b
    let delta = loc_b - loc_a;

    // interior points
    let interior = if delta.x().abs() % 3 == 0 && delta.y().abs() % 3 == 0 {
        let delta = delta.divide(3);
        let a = loc_a + delta;
        let b = loc_a + delta.multiply(2);
        println!("Found interior points for {} and {}", loc_a, loc_b);
        Left(once(a).chain(once(b)))
    } else {
        Right(empty())
    };

    // exterior points, ignoring bounding box
    let exterior = once(loc_a - delta).chain(once(loc_b + delta));

    interior.chain(exterior)
}

fn find_mega_antinodes(
    bounds: &BoundingBox2D<i32>,
    loc_a: Point2D<i32>,
    loc_b: Point2D<i32>,
) -> impl Iterator<Item = Point2D<i32>> + use<'_> {
    let mut delta = loc_b - loc_a;

    if delta.x().gcd(&delta.y()) != 1 {
        let gcd = delta.x().gcd(&delta.y());
        println!(
            "Found reducible line ({}) for {} and {}: {}",
            gcd, loc_a, loc_b, delta
        );
        delta = delta.divide(gcd);
    }

    let forward =
        successors(Some(loc_a), move |p| Some(*p + delta)).take_while(|p| bounds.contains(p));

    let backward = successors(Some(loc_b), move |p| Some(*p - delta))
        .take_while(|p| bounds.contains(p))
        .skip(1);

    forward.chain(backward)
}

fn find_all_antinodes<'a>(
    bounds: &'a BoundingBox2D<i32>,
    locs: &'a [Point2D<i32>],
) -> impl Iterator<Item = Point2D<i32>> + 'a {
    locs.iter()
        .combinations(2)
        .flat_map(|loc_pair| find_antinodes(*loc_pair[0], *loc_pair[1]))
        .filter(|node| bounds.contains(node))
}

fn find_all_mega_antinodes<'a>(
    bounds: &'a BoundingBox2D<i32>,
    locs: &'a [Point2D<i32>],
) -> impl Iterator<Item = Point2D<i32>> + 'a {
    locs.iter()
        .combinations(2)
        .flat_map(|loc_pair| find_mega_antinodes(bounds, *loc_pair[0], *loc_pair[1]))
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(
        input
            .antennae
            .iter()
            .flat_map(|(_, locs)| find_all_antinodes(&input.bounds, locs))
            .collect::<HashSet<_>>()
            .len() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    Some(
        input
            .antennae
            .iter()
            .flat_map(|(_, locs)| find_all_mega_antinodes(&input.bounds, locs))
            .collect::<HashSet<_>>()
            .len() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
