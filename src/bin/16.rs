use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::util::{
    direction::{Direction, DIRECTIONS},
    point::Point2D,
    DistanceState,
};
use grid::Grid;
use petgraph::{algo::dijkstra, graph::NodeIndex, visit::EdgeRef as _, Graph};
use smallvec::SmallVec;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let (graph, nodes) = build_graph(&input);

    let start = nodes[&Node {
        location: input.start,
        direction: Direction::East,
    }];

    let scores = dijkstra(&graph, start, None, |edge| *edge.weight());

    let min_score = DIRECTIONS
        .iter()
        .map(|&direction| {
            scores[&nodes[&Node {
                location: input.end,
                direction,
            }]]
        })
        .min()
        .unwrap();

    Some(min_score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let (graph, nodes) = build_graph(&input);
    let reverse_nodes = nodes
        .iter()
        .map(|(node, &index)| (index, node))
        .collect::<HashMap<_, _>>();

    let start = nodes[&Node {
        location: input.start,
        direction: Direction::East,
    }];

    let ends = DIRECTIONS.map(|direction| {
        nodes[&Node {
            location: input.end,
            direction,
        }]
    });

    let (_, nodes_in_shortest_path) = all_shortest_paths(&graph, start, &ends);

    let locations_in_shortest_path = nodes_in_shortest_path
        .iter()
        .map(|&node| reverse_nodes[&node].location)
        .collect::<HashSet<_>>();

    Some(locations_in_shortest_path.len() as u64)
}

fn all_shortest_paths(
    graph: &Graph<(), u64>,
    start: NodeIndex,
    ends: &[NodeIndex],
) -> (u64, HashSet<NodeIndex>) {
    let mut distances = HashMap::new();
    let mut previous = HashMap::new();
    let mut queue = BinaryHeap::new();

    distances.insert(start, 0);
    queue.push(DistanceState::new(0, start));

    while let Some(DistanceState {
        distance,
        state: node,
    }) = queue.pop()
    {
        for edge in graph.edges(node) {
            let neighbor = edge.target();
            let weight = *edge.weight();
            let neighbor_distance = distance + weight;

            let previous_distance = distances.get(&neighbor);

            if previous_distance.is_none() || previous_distance.unwrap() > &neighbor_distance {
                distances.insert(neighbor, neighbor_distance);
                previous.insert(neighbor, HashSet::from([node]));
                queue.push(DistanceState::new(neighbor_distance, neighbor));
            } else if previous_distance.unwrap() == &neighbor_distance {
                previous.get_mut(&neighbor).unwrap().insert(node);
                queue.push(DistanceState::new(neighbor_distance, neighbor));
            }
        }
    }

    let shortest_distance = ends.iter().map(|&end| distances[&end]).min().unwrap();
    let ends_with_shortest_paths = ends
        .iter()
        .filter(|&&end| distances[&end] == shortest_distance)
        .cloned()
        .collect::<SmallVec<[_; 4]>>();

    let mut nodes_in_shortest_path = HashSet::new();
    let mut queue = Vec::new();
    queue.extend(ends_with_shortest_paths);

    while let Some(item) = queue.pop() {
        nodes_in_shortest_path.insert(item);
        if item != start {
            queue.extend(previous[&item].iter().cloned());
        }
    }

    (shortest_distance, nodes_in_shortest_path)
}

struct Input {
    map: Grid<bool>,
    start: Point2D<isize>,
    end: Point2D<isize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    location: Point2D<isize>,
    direction: Direction,
}

fn build_graph(input: &Input) -> (Graph<(), u64>, HashMap<Node, NodeIndex>) {
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();

    for ((row, col), &is_clear) in input.map.indexed_iter() {
        if is_clear {
            for direction in DIRECTIONS {
                let index = graph.add_node(());
                nodes.insert(
                    Node {
                        location: Point2D::new(col as isize, row as isize),
                        direction,
                    },
                    index,
                );
            }
        }
    }

    for (node, &index) in nodes.iter() {
        let forward = node.location + node.direction.into();
        if let Some(&forward_index) = nodes.get(&Node {
            location: forward,
            direction: node.direction,
        }) {
            graph.add_edge(index, forward_index, 1);
        }

        let left_index = nodes
            .get(&Node {
                location: node.location,
                direction: node.direction.rotate_left(),
            })
            .unwrap();
        graph.add_edge(index, *left_index, 1000);

        let right_index = nodes
            .get(&Node {
                location: node.location,
                direction: node.direction.rotate_right(),
            })
            .unwrap();
        graph.add_edge(index, *right_index, 1000);
    }

    (graph, nodes)
}

fn parse_input(input: &str) -> Input {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().trim().len();

    let mut map = Grid::init(rows, cols, false);
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let y = rows - y - 1;
            map[(y, x)] = match ch {
                '#' => false,
                '.' => true,
                'S' => {
                    start = Some(Point2D::new(x as isize, y as isize));
                    true
                }
                'E' => {
                    end = Some(Point2D::new(x as isize, y as isize));
                    true
                }
                _ => panic!("Unexpected character in map input"),
            };
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
