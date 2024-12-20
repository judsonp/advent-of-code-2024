use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::util::{direction::DIRECTIONS, point::Point2D, DistanceState};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2, 100)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 20, 100)
}

fn solve(input: &str, allowed_distance: i32, saves: i32) -> Option<u64> {
    let input = parse_input(input);

    let path = shortest_path(&input.map, input.start, input.end).unwrap();

    let mut viable_cheats = 0;
    for cheat_start_idx in 0..path.len() {
        for cheat_end_idx in (cheat_start_idx + saves as usize)..path.len() {
            let cheat_start = path[cheat_start_idx];
            let cheat_end = path[cheat_end_idx];
            let cheat_distance = cheat_start.manhattan_distance(cheat_end);
            let cheat_saved_distance = (cheat_end_idx - cheat_start_idx) as i32 - cheat_distance;
            if cheat_distance <= allowed_distance && cheat_saved_distance >= saves {
                viable_cheats += 1;
            }
        }
    }

    Some(viable_cheats)
}

fn shortest_path(map: &Map, start: Node, end: Node) -> Option<Vec<Node>> {
    let paths = find_shortest_path(map, start, end)?;

    let mut path = Vec::new();
    let mut node = end;

    path.push(node);
    while node != start {
        node = paths[&node].state;
        path.push(node);
    }

    Some(path)
}

fn find_shortest_path(
    map: &Map,
    start: Node,
    end: Node,
) -> Option<HashMap<Node, DistanceState<u32, Node>>> {
    let mut state: HashMap<Node, DistanceState<u32, Node>> = HashMap::new();
    let mut queue: BinaryHeap<DistanceState<u32, Node>> = BinaryHeap::new();

    queue.push(DistanceState::new(0, start));
    state.insert(start, DistanceState::new(0, start));

    while let Some(item) = queue.pop() {
        let DistanceState {
            distance,
            state: node,
        } = item;
        for dir in DIRECTIONS {
            let neighbor = node + dir.into();
            if map.contains(&neighbor) {
                let neighbor_distance = state.get(&neighbor).map(|s| s.distance);
                if neighbor_distance.is_none() || neighbor_distance.unwrap() > distance + 1 {
                    state.insert(neighbor, DistanceState::new(distance + 1, node));
                    queue.push(DistanceState::new(distance + 1, neighbor));

                    if neighbor == end {
                        return Some(state);
                    }
                }
            }
        }
    }

    None
}

type Node = Point2D<i32>;
type Map = HashSet<Node>;

struct Input {
    map: Map,
    start: Node,
    end: Node,
}

fn parse_input(input: &str) -> Input {
    let mut map = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            let tile = match ch {
                '#' => false,
                '.' => true,
                'S' => {
                    start = Some((x, y).into());
                    true
                }
                'E' => {
                    end = Some((x, y).into());
                    true
                }
                _ => panic!("unexpected input"),
            };
            if tile {
                map.insert((x, y).into());
            }
        }
    }

    Input {
        map,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 2, 6);
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 20, 74);
        assert_eq!(result, Some(7));
    }
}
